#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Output, Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Receiver, Sender},
    mutex::Mutex,
};
use embassy_time::Timer;
use frame_data::frame::{Direction, Frame, FrameData};
use {defmt_rtt as _, panic_probe as _};

static CHANNEL: Channel<ThreadModeRawMutex, Direction, 2> = Channel::new();

const COL1: Frame = [0x10, 0x10, 0x10, 0x10, 0x10];
const COL2: Frame = [0x08, 0x08, 0x08, 0x08, 0x08];
const COL3: Frame = [0x04, 0x04, 0x04, 0x04, 0x04];
const COL4: Frame = [0x02, 0x02, 0x02, 0x02, 0x02];
const COL5: Frame = [0x01, 0x01, 0x01, 0x01, 0x01];

static FRAMES: &[Frame] = &[COL1, COL2, COL3, COL4, COL5];

type FrameDataType = Mutex<ThreadModeRawMutex, Option<FrameData<5>>>;

static FRAME_DATA: FrameDataType = Mutex::new(None);

//--------------------
// LED Refresh Task
//--------------------
#[embassy_executor::task]
async fn led_refresh_task(mut rows: [Output<'static>; 5], mut cols: [Output<'static>; 5]) {
    loop {
        // Read frame once per scan → lock only once
        let frame = {
            let frame_opt = FRAME_DATA.lock().await;
            frame_opt.as_ref().unwrap().current_frame().clone()
        };

        for row in 0..5 {
            for r in &mut rows {
                r.set_high();
            }

            rows[row].set_low();
            let row_bits = frame[row];

            for col in 0..5 {
                if (row_bits & (1 << (4 - col))) != 0 {
                    cols[col].set_low();
                } else {
                    cols[col].set_high();
                }
            }

            Timer::after_micros(300).await;
        }
    }
}

//--------------------
// Button Task
//--------------------
#[embassy_executor::task(pool_size = 2)]
async fn button_task(
    sender: Sender<'static, ThreadModeRawMutex, Direction, 2>,
    mut button: Input<'static>,
    value: Direction,
) {
    loop {
        button.wait_for_low().await;

        Timer::after_millis(20).await;

        if button.is_high() {
            sender.send(value).await;
        }
    }
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
            let mut frame_data_option = FRAME_DATA.lock().await;
            if let Some(frame_data) = frame_data_option.as_mut() {
                frame_data.move_index(button_pressed);
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let btn_a = Input::new(p.P0_14, Pull::Up);
    let btn_b = Input::new(p.P0_23, Pull::Up);
    let sender_a = CHANNEL.sender();
    let sender_b = CHANNEL.sender();
    let receiver = CHANNEL.receiver();

    let frame_data = FrameData::<5>::new(&FRAMES);
    {
        *(FRAME_DATA.lock().await) = Some(frame_data);
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

    spawner
        .spawn(button_task(sender_a, btn_a, Direction::Left))
        .expect("Failed to button A task");
    spawner
        .spawn(button_task(sender_b, btn_b, Direction::Right))
        .expect("Failed to spawn button B task");
    spawner
        .spawn(button_receiver(receiver))
        .expect("Failed to spawn button receiver task");

    spawner
        .spawn(led_refresh_task(rows, cols))
        .expect("Failed to spawn led refresh task");
}
