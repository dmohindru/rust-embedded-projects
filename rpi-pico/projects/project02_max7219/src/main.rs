#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Config, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use embedded_core::display_driver::Max7219;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// SPI type
type MySpi = Spi<'static, SPI0, Async>;

// SPI + CS shared container
// TODO: Bus in nothing more than a Mutex stored in Global storage
static SPI_BUS: StaticCell<Mutex<NoopRawMutex, MySpi>> = StaticCell::new();

// Max7219 Type
type Max7219Device = SpiDevice<'static, NoopRawMutex, MySpi, Output<'static>>;

#[embassy_executor::task]
async fn spi_init_task(mut spi_device: Max7219<Max7219Device, 8, 8>) {
    loop {
        spi_device.initialize().await.unwrap();
        info!("Initialization commands written");
        Timer::after_secs(10).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // SPI Config
    let mut spi_config = Config::default();
    spi_config.frequency = 8_000_000;
    spi_config.phase = embassy_rp::spi::Phase::CaptureOnFirstTransition;
    spi_config.polarity = embassy_rp::spi::Polarity::IdleLow;

    // Chip select pin
    let cs = Output::new(p.PIN_20, Level::High);

    // SPI Pins
    let clk = p.PIN_18;
    let tx = p.PIN_19;

    // SPI device
    let spi = Spi::new(p.SPI0, clk, tx, p.PIN_16, p.DMA_CH0, p.DMA_CH1, spi_config);

    // Shared bus
    // TODO: how did spi_but mutex got type of NoopRawMutex
    let spi_bus = Mutex::new(spi);
    let spi_bus = SPI_BUS.init(spi_bus);

    // // Device with CS handling
    let spi_device = SpiDevice::new(spi_bus, cs);

    // Max7219 Device
    let driver: Max7219<Max7219Device, 8, 8> = Max7219::new(spi_device);

    spawner
        .spawn(spi_init_task(driver))
        .expect("Failed to spawn spi_init_task");
}
