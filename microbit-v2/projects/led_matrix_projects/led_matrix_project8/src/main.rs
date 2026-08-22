#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::peripherals::SPI3;
use embassy_nrf::{bind_interrupts, spim};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use static_cell::StaticCell;

use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Receiver, Sender},
    mutex::Mutex,
};
use embedded_alloc::Heap;
use embedded_core::display_driver::Max7219;
use embedded_core::frame::{decode_frames, Direction, FrameCursorCircular};
use embedded_core::input_device::button::DebouncedButton;
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

const LED_MATRIX_ROWS: usize = 8;
const LED_MATRIX_COLS: usize = 8;
static CHANNEL: Channel<ThreadModeRawMutex, Direction, 2> = Channel::new();

static FRAME_BYTES: &[u8] = include_bytes!("../assets/poonam.bin");

type FrameCursorType =
    Mutex<ThreadModeRawMutex, Option<FrameCursorCircular<6, LED_MATRIX_ROWS, LED_MATRIX_COLS>>>;

type Max7219Device = SpiDevice<'static, NoopRawMutex, spim::Spim<'static, SPI3>, Output<'static>>;
type DisplayDriver = Max7219<Max7219Device, LED_MATRIX_ROWS, LED_MATRIX_COLS>;

static FRAME_CURSOR: FrameCursorType = Mutex::new(None);

// Create a type (Struct) that hold info about hardware interrupt and interrupt handler
bind_interrupts!(struct Irqs {
    SPIM3 => spim::InterruptHandler<SPI3>;
});

//--------------------
// Button Task
//--------------------
#[embassy_executor::task(pool_size = 2)]
async fn button_task(
    sender: Sender<'static, ThreadModeRawMutex, Direction, 2>,
    mut button: DebouncedButton<Input<'static>, embassy_time::Delay>,
    value: Direction,
) {
    button
        .wait(|| async {
            sender.send(value).await;
        })
        .await;
}

//--------------------
// Receiver Task
//--------------------
#[embassy_executor::task]
async fn button_receiver(
    receiver: Receiver<'static, ThreadModeRawMutex, Direction, 2>,
    mut max7219: DisplayDriver,
) {
    loop {
        let button_pressed = receiver.receive().await;
        info!("Button pressed {}", button_pressed);
        {
            let mut frame_data_option = FRAME_CURSOR.lock().await;
            if let Some(frame_data) = frame_data_option.as_mut() {
                frame_data.move_index(button_pressed);
                max7219
                    .write_bitmap(frame_data.current_frame())
                    .await
                    .unwrap();
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 8 KB heap — you choose the size
    const HEAP_SIZE: usize = 8 * 1024;
    static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    unsafe {
        ALLOCATOR.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
    }

    let p = embassy_nrf::init(Default::default());

    // Button initialization
    let btn_a = Input::new(p.P0_14, Pull::Up);
    let btn_b = Input::new(p.P0_23, Pull::Up);
    let debounced_button_a = DebouncedButton::new(btn_a, embassy_time::Delay, 20);
    let debounced_button_b = DebouncedButton::new(btn_b, embassy_time::Delay, 20);

    // SPI config
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M8;

    // SPI peripheral
    let spim = spim::Spim::new_txonly(p.SPI3, Irqs, p.P0_13, p.P0_15, config);

    // Chip select
    let cs = Output::new(p.P0_12, Level::High, OutputDrive::Standard);

    // Shared bus container
    static SPI_BUS: StaticCell<Mutex<NoopRawMutex, spim::Spim<SPI3>>> = StaticCell::new();
    let spi_bus = Mutex::new(spim);
    let spi_bus = SPI_BUS.init(spi_bus);

    // Device with CS handling
    let spi_device = SpiDevice::new(spi_bus, cs);

    // Max7219 driver
    let driver = Max7219::<Max7219Device, LED_MATRIX_ROWS, LED_MATRIX_COLS>::new(spi_device);

    let sender_a = CHANNEL.sender();
    let sender_b = CHANNEL.sender();
    let receiver = CHANNEL.receiver();

    let frames = decode_frames::<6, LED_MATRIX_ROWS, LED_MATRIX_COLS>(FRAME_BYTES);

    let frame_cursor = FrameCursorCircular::<6, LED_MATRIX_ROWS, LED_MATRIX_COLS>::new(&frames);
    {
        *(FRAME_CURSOR.lock().await) = Some(frame_cursor);
    }

    spawner
        .spawn(button_task(sender_a, debounced_button_a, Direction::Left))
        .expect("Failed to button A task");
    spawner
        .spawn(button_task(sender_b, debounced_button_b, Direction::Right))
        .expect("Failed to spawn button B task");
    spawner
        .spawn(button_receiver(receiver, driver))
        .expect("Failed to spawn button receiver task");
}
