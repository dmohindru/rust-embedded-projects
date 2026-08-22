#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, bind_interrupts, dma,
    exti::{self, ExtiInput},
    gpio::Pull,
    i2c::{self, Config as I2cConfig, I2c, Master},
    interrupt,
    mode::Async,
    peripherals,
    time::Hertz,
};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    mutex::{Mutex, MutexGuard},
};
use embassy_time::{Delay, Timer};
use embedded_core::{
    cursor::StepCursorCircular,
    display_driver::Ht16K33,
    frame::{Direction, FrameCursorCircular, decode_frames},
    input_device::button::DebouncedButton,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/*
daksh.bin --> daksh, num frames: 45
parth.bin --> parth, num frames: 45
dhruv.bin --> dhruv, num frames: 45
poonam.bin --> poonam, num frames: 54
*/

static FRAME_BYTES: &[u8] = include_bytes!("../assets/dhruv.bin");
const NUM_FRAMES: usize = 45;
static HT16K33_DEVICE_ADDR: u8 = 0x70;

const LED_MATRIX_ROWS: usize = 8;
const LED_MATRIX_COLS: usize = 8;

static DIRECTION: Mutex<ThreadModeRawMutex, Direction> = Mutex::new(Direction::Right);
static STEP_CURSOR: StaticCell<Mutex<ThreadModeRawMutex, StepCursorCircular>> = StaticCell::new();

// i2c type
type MyI2c = I2c<'static, Async, Master>;

// I2C shared container
static I2C_BUS: StaticCell<Mutex<ThreadModeRawMutex, MyI2c>> = StaticCell::new();

// HT16K33 Type
type Ht16K33Device = I2cDevice<'static, ThreadModeRawMutex, MyI2c>;

// Display Driver type
type DisplayDriver = Ht16K33<Ht16K33Device, LED_MATRIX_ROWS, LED_MATRIX_COLS>;

bind_interrupts!(struct I2cIrqs {
    // I2C Interrupts
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;

    // DMA Interrupts for TX and RX
    DMA1_CHANNEL6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_CHANNEL7 => dma::InterruptHandler<peripherals::DMA1_CH7>;
});

bind_interrupts!(struct LeftBtnIrqs {
    EXTI4 => exti::InterruptHandler<interrupt::typelevel::EXTI4>;
});

bind_interrupts!(struct RightBtnIrqs {
    EXTI3 => exti::InterruptHandler<interrupt::typelevel::EXTI3>;
});

//--------------------
// Left Button Task
//--------------------
#[embassy_executor::task]
async fn left_button_task(mut button: DebouncedButton<ExtiInput<'static, Async>, Delay>) {
    button
        .wait(|| async {
            {
                let mut direction: MutexGuard<'_, ThreadModeRawMutex, Direction> =
                    DIRECTION.lock().await;
                info!("Left button pressed. Direction {}", *direction);
                *direction = direction.toggle();
            }
        })
        .await;
}

//--------------------
// Right Button Task
//--------------------
#[embassy_executor::task]
async fn right_button_task(
    mut button: DebouncedButton<ExtiInput<'static, Async>, Delay>,
    cursor_mutex: &'static Mutex<ThreadModeRawMutex, StepCursorCircular>,
) {
    button
        .wait(|| async {
            {
                info!("Right button pressed");
                let mut step_cursor: MutexGuard<'_, ThreadModeRawMutex, StepCursorCircular> =
                    cursor_mutex.lock().await;
                step_cursor.move_step(Direction::Right);
            }
        })
        .await;
}

//--------------------
// Timer Task
//--------------------
#[embassy_executor::task]
async fn timer_task(
    mut display_driver: DisplayDriver,
    mut frame_cursor: FrameCursorCircular<NUM_FRAMES, LED_MATRIX_ROWS, LED_MATRIX_COLS>,
    step_cursor_mutex: &'static Mutex<ThreadModeRawMutex, StepCursorCircular>,
) {
    loop {
        // ---- 1. Read delay from step cursor ----
        let delay_ms = {
            let step_cursor: MutexGuard<'_, ThreadModeRawMutex, StepCursorCircular> =
                step_cursor_mutex.lock().await;
            step_cursor.current_value()
        };

        // ---- 2. Sleep (NO LOCKS HELD) ----
        Timer::after_millis(delay_ms as u64).await;
        info!("Running timer task delay {}ms", &delay_ms);

        // ---- 3. Read direction from mutex ----
        let direction = {
            let direction_guard: MutexGuard<'static, ThreadModeRawMutex, Direction> =
                DIRECTION.lock().await;
            *direction_guard
        };

        // ---- 4. Move the frame cursor ----
        frame_cursor.move_index(direction);

        // ---- 5. Render frame ----
        let frame = frame_cursor.current_frame();
        display_driver.write_bitmap(&frame).await.unwrap();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    // ----------------------I2C Config ----------------------
    // I2c Config
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);

    // I2c Pins
    let scl = p.PA9;
    let sda = p.PA10;

    // I2c Device
    let i2c = I2c::new(
        p.I2C1, scl, sda, p.DMA1_CH6, p.DMA1_CH7, I2cIrqs, i2c_config,
    );

    // Shared bus (I2c peripheral protected by Mutex)
    let i2c_bus_mutex = Mutex::new(i2c);
    let i2c_bus = I2C_BUS.init(i2c_bus_mutex);

    // I2c Device backed by bus
    let i2c_device = I2cDevice::new(i2c_bus);

    // Ht16K33 Device
    let mut driver: DisplayDriver = Ht16K33::new(i2c_device, HT16K33_DEVICE_ADDR);

    //-------------------------Left/Right Buttons --------------

    let btn_left = ExtiInput::new(p.PB4, p.EXTI4, Pull::Up, LeftBtnIrqs);
    let debounced_btn_left = DebouncedButton::new(btn_left, Delay, 20);

    let btn_right = ExtiInput::new(p.PB3, p.EXTI3, Pull::Up, RightBtnIrqs);
    let debounced_btn_right = DebouncedButton::new(btn_right, Delay, 20);

    //-----------------------Frame Data--------------------------
    let frames = decode_frames::<NUM_FRAMES, LED_MATRIX_ROWS, LED_MATRIX_COLS>(FRAME_BYTES);
    let frame_cursor: FrameCursorCircular<NUM_FRAMES, _, _> = FrameCursorCircular::new(&frames);

    driver.initialize().await.unwrap();
    info!("Initialization commands written");
    Timer::after_millis(100).await;

    let step_cursor = StepCursorCircular::new(2000, 500, 250, 7);
    let step_cursor_mutex = STEP_CURSOR.init(Mutex::new(step_cursor));

    driver
        .write_bitmap(frame_cursor.current_frame())
        .await
        .unwrap();

    spawner
        .spawn(left_button_task(debounced_btn_left))
        .expect("Failed to spawn left button task");

    spawner
        .spawn(right_button_task(debounced_btn_right, step_cursor_mutex))
        .expect("Failed to spawn right button task");

    spawner
        .spawn(timer_task(driver, frame_cursor, step_cursor_mutex))
        .expect("Failed to spawn receiver task");
}
