#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Async, Config, Spi};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Timer};
use embedded_core::button::DebouncedButton;
use embedded_core::display_driver::Max7219;
use embedded_core::frame::{decode_frames, Direction, FrameCursorCircular};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/*
daksh.bin --> daksh, num frames: 13
parth.bin --> parth, num frames: 13
dhruv.bin --> dhruv, num frames: 13
poonam.bin --> poonam, num frames: 22
*/
static FRAME_BYTES: &[u8] = include_bytes!("../assets/daksh.bin");
const NUM_FRAMES: usize = 13;

const LED_MATRIX_ROWS: usize = 8;
const LED_MATRIX_COLS: usize = 8;
const CHANNEL_SIZE: usize = 2;
static CHANNEL: Channel<ThreadModeRawMutex, Direction, CHANNEL_SIZE> = Channel::new();

// SPI type
type MySpi = Spi<'static, SPI0, Async>;

// SPI + CS shared container
static SPI_BUS: StaticCell<Mutex<ThreadModeRawMutex, MySpi>> = StaticCell::new();

// Max7219 Type
type Max7219Device = SpiDevice<'static, ThreadModeRawMutex, MySpi, Output<'static>>;

// Display Driver type
type DisplayDriver = Max7219<Max7219Device, LED_MATRIX_ROWS, LED_MATRIX_COLS>;

//--------------------
// Button Sender Task
//--------------------
#[embassy_executor::task(pool_size = 2)]
async fn button_sender_task(
    sender: Sender<'static, ThreadModeRawMutex, Direction, CHANNEL_SIZE>,
    mut button: DebouncedButton<Input<'static>, Delay>,
    direction: Direction,
) {
    button
        .wait(|| async {
            sender.send(direction).await;
        })
        .await;
}

//--------------------
// Receiver Task
//--------------------
#[embassy_executor::task]
async fn receiver_task(
    receiver: Receiver<'static, ThreadModeRawMutex, Direction, CHANNEL_SIZE>,
    mut display_driver: DisplayDriver,
    mut frame_cursor: FrameCursorCircular<NUM_FRAMES, LED_MATRIX_ROWS, LED_MATRIX_COLS>,
) {
    loop {
        let direction = receiver.receive().await;
        info!("Received direction: {}", direction);
        frame_cursor.move_index(direction);
        let frame = frame_cursor.current_frame();
        display_driver.write_bitmap(&frame).await.unwrap();
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
    let cs = Output::new(p.PIN_20, Level::High);

    // SPI Pins
    let clk = p.PIN_18;
    let tx = p.PIN_19;
    let rx = p.PIN_16;

    // SPI device
    // TODO: move to tx_only
    let spi = Spi::new(p.SPI0, clk, tx, rx, p.DMA_CH0, p.DMA_CH1, spi_config);

    // Shared bus (SPI peripheral protected by Mutex)
    let spi_bus_mutex = Mutex::new(spi);
    let spi_bus = SPI_BUS.init(spi_bus_mutex);

    // Device with CS handling
    let spi_device = SpiDevice::new(spi_bus, cs);

    // Max7219 Device
    let mut driver: DisplayDriver = Max7219::new(spi_device);
    // ------------ SPI Config Ends --------------

    // ------------ Left/Right buttons --------------
    let left_btn = Input::new(p.PIN_15, Pull::Up);
    let right_btn = Input::new(p.PIN_21, Pull::Up);
    let debounced_left_btn = DebouncedButton::new(left_btn, Delay, 20);
    let debounced_right_btn = DebouncedButton::new(right_btn, Delay, 20);

    //--------Frame Data-----------
    let frames = decode_frames::<NUM_FRAMES, LED_MATRIX_ROWS, LED_MATRIX_COLS>(FRAME_BYTES);
    let frame_cursor = FrameCursorCircular::new(&frames);

    driver.initialize().await.unwrap();
    info!("Initialization commands written");
    Timer::after_millis(100).await;

    driver
        .write_bitmap(frame_cursor.current_frame())
        .await
        .unwrap();

    let sender_a = CHANNEL.sender();
    let sender_b = CHANNEL.sender();
    let receiver = CHANNEL.receiver();

    spawner
        .spawn(button_sender_task(
            sender_a,
            debounced_left_btn,
            Direction::Left,
        ))
        .expect("Failed to spawn left button task");

    spawner
        .spawn(button_sender_task(
            sender_b,
            debounced_right_btn,
            Direction::Right,
        ))
        .expect("Failed to spawn right button task");

    spawner
        .spawn(receiver_task(receiver, driver, frame_cursor))
        .expect("Failed to spawn receiver task");
}
