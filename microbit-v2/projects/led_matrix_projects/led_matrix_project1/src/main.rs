#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use common_utils::display_driver::{EmbassyDelay, MicroBitLedDriver};
use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Output, Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Receiver, Sender},
    mutex::Mutex,
};
use embedded_alloc::Heap;
use embedded_core::button::DebouncedButton;
use embedded_core::frame::{Direction, Frame, FrameCursor};
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

static CHANNEL: Channel<ThreadModeRawMutex, Direction, 2> = Channel::new();

const COL1: [u32; 5] = [0x10, 0x10, 0x10, 0x10, 0x10];
const COL2: [u32; 5] = [0x08, 0x08, 0x08, 0x08, 0x08];
const COL3: [u32; 5] = [0x04, 0x04, 0x04, 0x04, 0x04];
const COL4: [u32; 5] = [0x02, 0x02, 0x02, 0x02, 0x02];
const COL5: [u32; 5] = [0x01, 0x01, 0x01, 0x01, 0x01];

type FrameCursorType = Mutex<ThreadModeRawMutex, Option<FrameCursor<5, 5, 5>>>;

static FRAME_CURSOR: FrameCursorType = Mutex::new(None);

//--------------------
// Led Refresh task
//--------------------
#[embassy_executor::task]
async fn led_refresh_task(mut driver: MicroBitLedDriver<Output<'static>, EmbassyDelay>) {
    loop {
        // Read frame once per scan → lock only once
        let frame = {
            let frame_opt = FRAME_CURSOR.lock().await;
            frame_opt.as_ref().unwrap().current_frame().clone()
        };

        driver.render(&frame).await;
    }
}

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
async fn button_receiver(receiver: Receiver<'static, ThreadModeRawMutex, Direction, 2>) {
    loop {
        let button_pressed = receiver.receive().await;
        info!("Button pressed {}", button_pressed);
        {
            let mut frame_data_option = FRAME_CURSOR.lock().await;
            if let Some(frame_data) = frame_data_option.as_mut() {
                frame_data.move_index(button_pressed);
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
    let btn_a = Input::new(p.P0_14, Pull::Up);
    let btn_b = Input::new(p.P0_23, Pull::Up);
    let debounced_button_a = DebouncedButton::new(btn_a, embassy_time::Delay, 20);
    let debounced_button_b = DebouncedButton::new(btn_b, embassy_time::Delay, 20);
    let sender_a = CHANNEL.sender();
    let sender_b = CHANNEL.sender();
    let receiver = CHANNEL.receiver();

    let f1: Frame<5, 5> = Frame::<5, 5>::new(COL1);
    let f2: Frame<5, 5> = Frame::<5, 5>::new(COL2);
    let f3: Frame<5, 5> = Frame::<5, 5>::new(COL3);
    let f4: Frame<5, 5> = Frame::<5, 5>::new(COL4);
    let f5: Frame<5, 5> = Frame::<5, 5>::new(COL5);
    let frames = [f1, f2, f3, f4, f5];
    let frame_cursor = FrameCursor::<5, 5, 5>::new(&frames);
    {
        *(FRAME_CURSOR.lock().await) = Some(frame_cursor);
    }

    // -----------------
    // LED matrix pins
    // -----------------
    let rows = [
        Output::new(
            p.P0_21,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_22,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_15,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_24,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_19,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
    ];

    let cols = [
        Output::new(
            p.P0_28,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_11,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_31,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P1_05,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_30,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
    ];

    let delay = EmbassyDelay;

    let led_driver = MicroBitLedDriver::new(rows, cols, delay);

    spawner
        .spawn(button_task(sender_a, debounced_button_a, Direction::Left))
        .expect("Failed to button A task");
    spawner
        .spawn(button_task(sender_b, debounced_button_b, Direction::Right))
        .expect("Failed to spawn button B task");
    spawner
        .spawn(button_receiver(receiver))
        .expect("Failed to spawn button receiver task");

    spawner
        .spawn(led_refresh_task(led_driver))
        .expect("Failed to spawn led refresh task");
}
