#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    mutex::{Mutex, MutexGuard},
};
use embassy_time::{Delay, Timer};
use embedded_core::frame::Direction;
use embedded_core::input_device::button::DebouncedButton;
use {defmt_rtt as _, panic_probe as _};

type ButtonStateType = Mutex<ThreadModeRawMutex, Option<Direction>>;
static BUTTON_STATE: ButtonStateType = Mutex::new(None);

#[embassy_executor::task]
async fn log_button_state_task() {
    loop {
        /*
        1. The task asynchronously waits until it can acquire the mutex.
        2. Once acquired, it gets a guard to the protected data, dereferences it, and copies the value out.
        3. When the block ends, the guard is dropped automatically, releasing the mutex so other tasks can use it.
         */
        let button_state_opt = {
            let button_opt: MutexGuard<'_, ThreadModeRawMutex, Option<Direction>> =
                BUTTON_STATE.lock().await;
            *button_opt
        };
        match button_state_opt {
            None => info!("No button pressed"),
            Some(button_pressed) => match button_pressed {
                Direction::Left => info!("Left button pressed"),
                Direction::Right => info!("Right button pressed"),
            },
        }
        Timer::after_secs(2).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn button_task(mut button: DebouncedButton<Input<'static>, Delay>, direction: Direction) {
    button
        .wait(|| async {
            {
                let mut button_state: MutexGuard<'_, ThreadModeRawMutex, Option<Direction>> =
                    BUTTON_STATE.lock().await;
                *button_state = Some(direction);
                info!("Moving state to {}", direction)
            }
        })
        .await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let btn_a = Input::new(p.PIN_15, Pull::Up);
    let btn_b = Input::new(p.PIN_16, Pull::Up);
    let debounced_btn_a = DebouncedButton::new(btn_a, Delay, 20);
    let debounced_btn_b = DebouncedButton::new(btn_b, Delay, 20);

    spawner
        .spawn(log_button_state_task())
        .expect("Failed to spawn log button state task");

    spawner
        .spawn(button_task(debounced_btn_a, Direction::Left))
        .expect("Failed to spawn button a task");

    spawner
        .spawn(button_task(debounced_btn_b, Direction::Right))
        .expect("Failed to spawn button b task");
}
