#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Config, Spi};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Timer};
use embedded_core::button::DebouncedButton;
use embedded_core::shift_register::Hc595;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/*
Connection details
SER     -> GP19 (SPI0 TX)
SRCLK   -> GP18 (SPI0 SCK)
RCLK    -> GP20 (GPIO)
*/

// SPI type
type MySpi = Spi<'static, SPI0, Async>;

// SPI + CS shared container
static SPI_BUS: StaticCell<Mutex<ThreadModeRawMutex, MySpi>> = StaticCell::new();

// HC959 Spi Type
type Hc595SpiDevice = SpiDevice<'static, ThreadModeRawMutex, MySpi, Output<'static>>;

// HC595 Device type
type Hc595Device = Hc595<Hc595SpiDevice, Output<'static>, 1>;

//--------------------
// Left Button Task
//--------------------
#[embassy_executor::task]
async fn left_button_task(mut button: DebouncedButton<Input<'static>, Delay>) {
    button
        .wait(|| async {
            {
                info!("Left button pressed");
            }
        })
        .await;
}

//--------------------
// Right Button Task
//--------------------
#[embassy_executor::task]
async fn right_button_task(mut button: DebouncedButton<Input<'static>, Delay>) {
    button
        .wait(|| async {
            {
                info!("Right button pressed");
            }
        })
        .await;
}

//--------------------
// Timer Task
//--------------------
#[embassy_executor::task]
async fn timer_task(mut hc595_device: Hc595Device, delay_ms: u64) {
    let mut counter: u8 = 0;
    loop {
        info!("Writing counter value {}", counter);
        hc595_device.write(&[counter]).await.unwrap();
        Timer::after_millis(delay_ms).await;
        counter = counter.wrapping_add(1);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // ------------ SPI Config --------------
    // SPI Config
    let mut spi_config = Config::default();
    spi_config.frequency = 8_000_000;

    // Chip select pin
    // TODO: Not really used in this example
    let cs = Output::new(p.PIN_14, Level::High);

    // Latch pin
    let latch = Output::new(p.PIN_20, Level::High);

    // SPI Pins
    let clk = p.PIN_18;
    let tx = p.PIN_19;

    // SPI device
    let spi = Spi::new_txonly(p.SPI0, clk, tx, p.DMA_CH0, spi_config);

    // Shared bus (SPI peripheral protected by Mutex)
    let spi_bus_mutex = Mutex::new(spi);
    let spi_bus = SPI_BUS.init(spi_bus_mutex);

    // Device with CS handling
    let spi_device = SpiDevice::new(spi_bus, cs);

    // LH595 Device
    let hc595_device: Hc595Device = Hc595::new(spi_device, latch, None, None).unwrap();
    // ------------ SPI Config Ends --------------

    // ------------ Left/Right buttons --------------
    let left_btn = Input::new(p.PIN_15, Pull::Up);
    let right_btn = Input::new(p.PIN_21, Pull::Up);
    let debounced_left_btn = DebouncedButton::new(left_btn, Delay, 20);
    let debounced_right_btn = DebouncedButton::new(right_btn, Delay, 20);

    spawner
        .spawn(left_button_task(debounced_left_btn))
        .expect("Failed to spawn left button task");

    spawner
        .spawn(right_button_task(debounced_right_btn))
        .expect("Failed to spawn right button task");

    spawner
        .spawn(timer_task(hc595_device, 500))
        .expect("Failed to spawn receiver task");
}
