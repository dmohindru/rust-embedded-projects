#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{AnyPin, Input, Output, Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[derive(defmt::Format, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

static CHANNEL: Channel<ThreadModeRawMutex, Direction, 2> = Channel::new();

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

    spawner
        .spawn(button_task(sender_a, btn_a, Direction::Left))
        .expect("Failed to button A task");
    spawner
        .spawn(button_task(sender_b, btn_b, Direction::Right))
        .expect("Failed to spawn button B task");
    spawner
        .spawn(button_receiver(receiver))
        .expect("Failed to spawn button receiver task");
}

/*
TODO
1. Create a data structure
    1.1 To hold all frame buffer data to be used by application. In this case list of 5 frame data (two dimensional array of unsigned int or even bool)
    1.2 function to get the current active frame
    1.3 function to move the pointer to other frames as per business logic avoiding data race condition
2. Create a frame rendered that takes the current frame data and renders it

*/
