#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Config, Spi};
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::{Delay, Timer};
use embedded_core::input_device::button::DebouncedButton;
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
type Hc595Device = Hc595<Hc595SpiDevice, Output<'static>, 2>;

// Channel for sending enabling/disabling HC595 output
static ENABLE_CHANNEL: Channel<ThreadModeRawMutex, bool, 1> = Channel::new();
static ENABLE: Mutex<ThreadModeRawMutex, bool> = Mutex::new(true);

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
async fn right_button_task(
    sender: Sender<'static, ThreadModeRawMutex, bool, 1>,
    mut button: DebouncedButton<Input<'static>, Delay>,
) {
    button
        .wait(|| async {
            {
                let mut enable: MutexGuard<'_, ThreadModeRawMutex, bool> = ENABLE.lock().await;
                *enable = !*enable;
                sender.send(*enable).await;
                info!("Right button pressed. Sending value {}", *enable);
            }
        })
        .await;
}

//--------------------
// Timer Task
//--------------------
#[embassy_executor::task]
async fn timer_task(
    enable_receiver: Receiver<'static, ThreadModeRawMutex, bool, 1>,
    mut hc595_device: Hc595Device,
    delay_ms: u64,
) {
    let mut pattern: u16 = 0xAAAA;
    loop {
        match select(Timer::after_millis(delay_ms), enable_receiver.receive()).await {
            Either::First(_) => {
                // Timer elapsed
                let data: [u8; 2] = pattern.to_le_bytes();
                info!("Writing counter value {}", &pattern);
                hc595_device.write(&data).await.unwrap();
                Timer::after_millis(delay_ms).await;
                pattern = !pattern;
            }

            Either::Second(new_state) => {
                if new_state {
                    hc595_device.enable().unwrap();
                    info!("Display enabled");
                } else {
                    hc595_device.disable().unwrap();
                    info!("Display disabled");
                }
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    //------------- Sender/Receivers --------
    let enable_sender = ENABLE_CHANNEL.sender();
    let enable_receiver = ENABLE_CHANNEL.receiver();

    // ------------ SPI Config --------------
    // SPI Config
    let mut spi_config = Config::default();
    spi_config.frequency = 1_000_000;

    // Chip select pin
    // TODO: Not really used in this example
    let cs = Output::new(p.PIN_14, Level::High);

    // Latch pin active low
    let latch = Output::new(p.PIN_20, Level::Low);

    // Output enable pin active low
    let output_enable = Output::new(p.PIN_22, Level::Low);

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
    let hc595_device: Hc595Device =
        Hc595::new(spi_device, latch, Some(output_enable), None).unwrap();
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
        .spawn(right_button_task(enable_sender, debounced_right_btn))
        .expect("Failed to spawn right button task");

    spawner
        .spawn(timer_task(enable_receiver, hc595_device, 500))
        .expect("Failed to spawn receiver task");
}
