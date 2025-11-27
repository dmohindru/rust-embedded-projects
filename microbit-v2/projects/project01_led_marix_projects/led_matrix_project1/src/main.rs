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

enum Direction {
    Left,
    Right,
}

static CHANNEL: Channel<ThreadModeRawMutex, Direction, 2> = Channel::new();

// Button task that would detect button press and send a message on a channel
// Pool 2 required for button A, button B
// Params
// 1. Button pin
// 2. Channel sender
// 3. Value to send
#[embassy_executor::task]
async fn button_task(
    sender: Sender<'static, ThreadModeRawMutex, Direction, 2>,
    button: Input<'static>,
    value: Direction,
) {
    loop {
        button.wait_for_high().await;

        Timer::after_millis(20).await;

        if button.is_high() {
            sender.send(value).await;
        }

        button.wait_for_low().await;
    }
}

// Receiver task to receive button press and do its business logic
// Params
// 1. Channel Receiver
#[embassy_executor::task]
async fn button_receiver(receiver: Receiver<'static, ThreadModeRawMutex, Direction, 2>) {
    loop {
        let button_pressed = receiver.receive().await;

        info!("Slow task: Hello from microbit v2! (2s interval)");
        Timer::after(Duration::from_secs(2)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let btn_a = Input::new(p.P0_14, Pull::Up);
    let btn_b = Input::new(p.P0_23, Pull::Up);
    let sender = CHANNEL.sender();
    let receiver = CHANNEL.recv();

    spawner.spawn(button_task(sender, btn_a, Direction::Left));
    spawner.spawn(button_task(sender, btn_b, Direction::Left));
}

/*
TODO
1. Create a data structure
    1.1 To hold all frame buffer data to be used by application. In this case list of 5 frame data (two dimensional array of unsigned int or even bool)
    1.2 function to get the current active frame
    1.3 function to move the pointer to other frames as per business logic avoiding data race condition
2. Create a frame rendered that takes the current frame data and renders it

*/
