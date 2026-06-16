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
    button::DebouncedButton, cursor::StepCursorCircular, display_driver::Ht16K33, frame::Direction,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/*
daksh.bin --> daksh, num frames: 45
parth.bin --> parth, num frames: 45
dhruv.bin --> dhruv, num frames: 45
poonam.bin --> poonam, num frames: 54
*/

static FRAME_BYTES: &[u8] = include_bytes!("../assets/poonam.bin");
const NUM_FRAMES: usize = 54;
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

// #[embassy_executor::task]
// async fn log_button_state() {
//     loop {
//         /*
//         1. The task asynchronously waits until it can acquire the mutex.
//         2. Once acquired, it gets a guard to the protected data, dereferences it, and copies the value out.
//         3. When the block ends, the guard is dropped automatically, releasing the mutex so other tasks can use it.
//          */
//         let button_state = {
//             let button_mutex: MutexGuard<'_, ThreadModeRawMutex, Direction> =
//                 BUTTON_STATE.lock().await;
//             *button_mutex
//         };
//         info!("Direction: {}", button_state);
//         Timer::after_secs(1).await;
//     }
// }

// #[embassy_executor::task(pool_size = 2)]
// async fn button_task(mut button: DebouncedButton<ExtiInput<'static>, Delay>, direction: Direction) {
//     button
//         .wait(|| async {
//             let mut button_state: MutexGuard<'_, ThreadModeRawMutex, Direction> =
//                 BUTTON_STATE.lock().await;
//             *button_state = direction;
//             info!("Moving state to {}", direction);
//         })
//         .await;
// }

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

    // ------------------------I2C Config ends ------------------

    let btn_left = ExtiInput::new(p.PB4, p.EXTI4, Pull::Up, LeftBtnIrqs);
    let debounced_btn_left = DebouncedButton::new(btn_left, Delay, 20);

    let btn_right = ExtiInput::new(p.PB3, p.EXTI3, Pull::Up, RightBtnIrqs);
    let debounced_btn_right = DebouncedButton::new(btn_right, Delay, 20);

    // spawner
    //     .spawn(log_button_state())
    //     .expect("Failed to spawn log button state task");

    // spawner
    //     .spawn(button_task(debounced_btn_left, Direction::Left))
    //     .expect("Failed to spawn left button task");

    // spawner
    //     .spawn(button_task(debounced_btn_right, Direction::Right))
    //     .expect("Failed to spawn right button task");
}
