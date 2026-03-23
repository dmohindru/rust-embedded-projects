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
use embedded_core::frame::{decode_frames, Direction, Frame, FrameCursorCircular};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const LED_MATRIX_ROWS: usize = 8;
const LED_MATRIX_COLS: usize = 8;
static CHANNEL: Channel<ThreadModeRawMutex, Direction, 2> = Channel::new();

type FrameCursorType =
    Mutex<ThreadModeRawMutex, Option<FrameCursorCircular<6, LED_MATRIX_ROWS, LED_MATRIX_COLS>>>;
static FRAME_CURSOR: FrameCursorType = Mutex::new(None);

static FRAME_BYTES: &[u8] = include_bytes!("../assets/poonam.bin");

// SPI type
type MySpi = Spi<'static, SPI0, Async>;

// TODO: Research between ThreadModeRawMutex and NoopRawMutex
// SPI + CS shared container
static SPI_BUS: StaticCell<Mutex<ThreadModeRawMutex, MySpi>> = StaticCell::new();

// Max7219 Type
type Max7219Device = SpiDevice<'static, ThreadModeRawMutex, MySpi, Output<'static>>;

// Display Driver type
type DisplayDriver = Max7219<Max7219Device, LED_MATRIX_ROWS, LED_MATRIX_COLS>;

#[embassy_executor::task]
async fn spi_init_task(mut spi_device: Max7219<Max7219Device, 8, 8>) {
    let frame_data: [u32; 8] = [0xF0, 0x0F, 0xE0, 0x0E, 0xD0, 0x0D, 0xC0, 0x0C];
    let frame: Frame<8, 8> = Frame::new(frame_data);

    loop {
        spi_device.write_bitmap(&frame).await.unwrap();
        info!("Frame data written");
        Timer::after_secs(5).await;
    }
}

//--------------------
// Button Sender Task
//--------------------
#[embassy_executor::task(pool_size = 2)]
async fn button_sender_task(
    sender: Sender<'static, ThreadModeRawMutex, Direction, 2>,
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
    receiver: Receiver<'static, ThreadModeRawMutex, Direction, 2>,
    mut display_driver: DisplayDriver,
) {
    loop {
        let direction = receiver.receive().await;
        info!("Received direction: {}", direction);
        let frame = {
            let mut frame_data_option = FRAME_CURSOR.lock().await;
            if let Some(frame_data) = frame_data_option.as_mut() {
                frame_data.move_index(direction);
                frame_data.current_frame().clone()
            } else {
                continue;
            }
        };
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
    let frames = decode_frames::<6, LED_MATRIX_ROWS, LED_MATRIX_COLS>(FRAME_BYTES);
    let frame_cursor = FrameCursorCircular::new(&frames);
    //--------Frame Data Ends-----------

    driver.initialize().await.unwrap();
    info!("Initialization commands written");
    Timer::after_millis(100).await;
    // TODO: Check if its safe to use frame_cursor here without a mutex
    driver
        .write_bitmap(frame_cursor.current_frame())
        .await
        .unwrap();
    // Set mutex to be used for other tasks
    {
        *(FRAME_CURSOR.lock().await) = Some(frame_cursor);
    }

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
        .spawn(receiver_task(receiver, driver))
        .expect("Failed to spawn receiver task");
}
