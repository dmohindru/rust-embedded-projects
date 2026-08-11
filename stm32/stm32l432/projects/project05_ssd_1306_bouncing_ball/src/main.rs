#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
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
    channel::{Channel, Receiver, Sender},
    mutex::Mutex,
};
use embassy_time::{Delay, Timer};
use embedded_core::{
    button::DebouncedButton, display_driver::Ssd1306_128x64, service::bouncing_ball::BouncingBall,
};
use embedded_graphics::pixelcolor::BinaryColor;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const CHANNEL_SIZE: usize = 1;
static CHANNEL: Channel<ThreadModeRawMutex, (), CHANNEL_SIZE> = Channel::new();

static DEVICE_ADDR: u8 = 0x3C;

// i2c type
type MyI2c = I2c<'static, Async, Master>;

// I2C shared container
static I2C_BUS: StaticCell<Mutex<ThreadModeRawMutex, MyI2c>> = StaticCell::new();

// I2C Device Type
type MyI2cDevice = I2cDevice<'static, ThreadModeRawMutex, MyI2c>;

// Display Driver type
type DisplayDriver = Ssd1306_128x64<MyI2cDevice>;

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

// --------------------
// Left Button Task
// --------------------
#[embassy_executor::task]
async fn left_button_task(
    mut button: DebouncedButton<ExtiInput<'static, Async>, Delay>,
    sender: Sender<'static, ThreadModeRawMutex, (), CHANNEL_SIZE>,
) {
    button
        .wait(|| async {
            {
                sender.send(()).await;
                info!("Sending Animation Toggle command");
            }
        })
        .await;
}

#[embassy_executor::task]
async fn animation_task(
    receiver: Receiver<'static, ThreadModeRawMutex, (), CHANNEL_SIZE>,
    mut bouncing_ball: BouncingBall<DisplayDriver, 128, 64, BinaryColor>,
) {
    let mut animation_running = true;
    loop {
        let timer_fut = Timer::after_millis(150);
        let receiver_fut = receiver.receive();

        match select(receiver_fut, timer_fut).await {
            Either::First(_) => {
                animation_running = !animation_running;
            }
            Either::Second(_) => {
                if animation_running {
                    bouncing_ball.update().await.unwrap();
                }
            }
        }
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
    let mut driver: DisplayDriver = Ssd1306_128x64::new(i2c_device, DEVICE_ADDR);

    //-------------------------Left Buttons --------------

    let btn_left = ExtiInput::new(p.PB4, p.EXTI4, Pull::Up, LeftBtnIrqs);
    let debounced_btn_left = DebouncedButton::new(btn_left, Delay, 20);

    // -----------------------Sender and receivers--------------------------
    let left_button_sender = CHANNEL.sender();
    let receiver = CHANNEL.receiver();

    driver.initialize().await.unwrap();
    info!("Initialization commands written");

    let bouncing_ball = BouncingBall::<_, 128, 64, BinaryColor>::new(
        driver,
        10,
        5,
        BinaryColor::On,
        BinaryColor::Off,
    );

    spawner
        .spawn(left_button_task(debounced_btn_left, left_button_sender))
        .expect("Failed to spawn left button task");

    spawner
        .spawn(animation_task(receiver, bouncing_ball))
        .expect("Failed to spawn receiver task");
}
