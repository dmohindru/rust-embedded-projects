#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, bind_interrupts, dma,
    i2c::{self, Config as I2cConfig, I2c, Master},
    mode::Async,
    peripherals,
    time::Hertz,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use embedded_core::input_device::nunchuk::{Nunchuk, NunchukType};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

static DEVICE_ADDR: u8 = 0x52;

// i2c type
type MyI2c = I2c<'static, Async, Master>;

// I2C shared container
static I2C_BUS: StaticCell<Mutex<ThreadModeRawMutex, MyI2c>> = StaticCell::new();

// I2C Device Type
type MyI2cDevice = I2cDevice<'static, ThreadModeRawMutex, MyI2c>;

// Nunchuk Driver type
type NunchukDriver = Nunchuk<MyI2cDevice>;

bind_interrupts!(struct I2cIrqs {
    // I2C Interrupts
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;

    // DMA Interrupts for TX and RX
    DMA1_CHANNEL6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_CHANNEL7 => dma::InterruptHandler<peripherals::DMA1_CH7>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
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

    // Nunchuk Device
    let mut nunchuk_device: NunchukDriver =
        Nunchuk::new(i2c_device, DEVICE_ADDR, NunchukType::Black);

    nunchuk_device.initialize().await.unwrap();
    info!("Nunchuk Initialization commands written");

    loop {
        Timer::after_millis(150).await;
        let nunchuk_report = nunchuk_device.poll().await.unwrap();
        info!(
            "x_axis: {}, y_axis: {}, x_acceleration: {}, y_acceleration: {}, z_acceleration: {}, c_button_pressed: {}, z_button_pressed: {}",
            nunchuk_report.x_axis,
            nunchuk_report.y_axis,
            nunchuk_report.x_acceleration,
            nunchuk_report.y_acceleration,
            nunchuk_report.z_acceleration,
            nunchuk_report.c_button_pressed,
            nunchuk_report.z_button_pressed
        )
    }
}
