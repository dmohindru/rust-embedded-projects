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
    channel::{Channel, Receiver, Sender},
    mutex::Mutex,
};
use embassy_time::Delay;
use embedded_core::{button::DebouncedButton, display_driver::Ssd1306_128x64};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

enum Ops {
    Toggle,
    Draw,
}
const CHANNEL_SIZE: usize = 3;
static CHANNEL: Channel<ThreadModeRawMutex, Ops, CHANNEL_SIZE> = Channel::new();

// TODO Fix this address
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

bind_interrupts!(struct RightBtnIrqs {
    EXTI3 => exti::InterruptHandler<interrupt::typelevel::EXTI3>;
});

//--------------------
// Left Button Task
//--------------------
#[embassy_executor::task]
async fn left_button_task(
    mut button: DebouncedButton<ExtiInput<'static, Async>, Delay>,
    sender: Sender<'static, ThreadModeRawMutex, Ops, CHANNEL_SIZE>,
) {
    button
        .wait(|| async {
            {
                sender.send(Ops::Toggle).await;
                info!("Sending Toggle command");
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
    sender: Sender<'static, ThreadModeRawMutex, Ops, CHANNEL_SIZE>,
) {
    button
        .wait(|| async {
            {
                sender.send(Ops::Draw).await;
                info!("Sending Draw command");
            }
        })
        .await;
}

#[embassy_executor::task]
async fn driver_receiver_task(
    receiver: Receiver<'static, ThreadModeRawMutex, Ops, CHANNEL_SIZE>,
    mut display_driver: DisplayDriver,
) {
    let mut set_rectangle_flag = true;
    loop {
        let ops = receiver.receive().await;
        match ops {
            Ops::Toggle => display_driver.invert_display().await.unwrap(),
            Ops::Draw => {
                let x = 60;
                let y = 30;
                let width = 20;

                for dx in 0..width {
                    for dy in 0..width {
                        if set_rectangle_flag {
                            display_driver.set_pixel(x + dx, y + dy);
                        } else {
                            display_driver.clear_pixel(x + dx, y + dy);
                        }
                    }
                }
                set_rectangle_flag = !set_rectangle_flag;
                display_driver.flush().await.unwrap();
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

    //-------------------------Left/Right Buttons --------------

    let btn_left = ExtiInput::new(p.PB4, p.EXTI4, Pull::Up, LeftBtnIrqs);
    let debounced_btn_left = DebouncedButton::new(btn_left, Delay, 20);

    let btn_right = ExtiInput::new(p.PB3, p.EXTI3, Pull::Up, RightBtnIrqs);
    let debounced_btn_right = DebouncedButton::new(btn_right, Delay, 20);

    //-----------------------Sender and receivers--------------------------
    let sender_left = CHANNEL.sender();
    let sender_right = CHANNEL.sender();
    let receiver = CHANNEL.receiver();

    driver.initialize().await.unwrap();
    info!("Initialization commands written");

    // Draw some pixels for fun
    driver.set_pixel(0, 0);
    driver.set_pixel(127, 0);
    driver.set_pixel(127, 63);
    driver.set_pixel(0, 63);
    driver.flush().await.unwrap();

    sender_right.send(Ops::Draw).await;

    spawner
        .spawn(left_button_task(debounced_btn_left, sender_left))
        .expect("Failed to spawn left button task");

    spawner
        .spawn(right_button_task(debounced_btn_right, sender_right))
        .expect("Failed to spawn right button task");

    spawner
        .spawn(driver_receiver_task(receiver, driver))
        .expect("Failed to spawn receiver task");
}
