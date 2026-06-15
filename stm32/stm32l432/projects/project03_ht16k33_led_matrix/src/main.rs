#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{Config, exti::ExtiInput, gpio::Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    mutex::{Mutex, MutexGuard},
};
use embassy_time::{Delay, Timer};
use embedded_core::{button::DebouncedButton, frame::Direction};
use {defmt_rtt as _, panic_probe as _};

type ButtonStateType = Mutex<ThreadModeRawMutex, Direction>;
static BUTTON_STATE: ButtonStateType = Mutex::new(Direction::Left);

#[embassy_executor::task]
async fn log_button_state() {
    loop {
        /*
        1. The task asynchronously waits until it can acquire the mutex.
        2. Once acquired, it gets a guard to the protected data, dereferences it, and copies the value out.
        3. When the block ends, the guard is dropped automatically, releasing the mutex so other tasks can use it.
         */
        let button_state = {
            let button_mutex: MutexGuard<'_, ThreadModeRawMutex, Direction> =
                BUTTON_STATE.lock().await;
            *button_mutex
        };
        info!("Direction: {}", button_state);
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn button_task(mut button: DebouncedButton<ExtiInput<'static>, Delay>, direction: Direction) {
    button
        .wait(|| async {
            let mut button_state: MutexGuard<'_, ThreadModeRawMutex, Direction> =
                BUTTON_STATE.lock().await;
            *button_state = direction;
            info!("Moving state to {}", direction);
        })
        .await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    let btn_left = ExtiInput::new(p.PB4, p.EXTI4, Pull::Up);
    let debounced_btn_left = DebouncedButton::new(btn_left, Delay, 20);

    let btn_right = ExtiInput::new(p.PB3, p.EXTI3, Pull::Up);
    let debounced_btn_right = DebouncedButton::new(btn_right, Delay, 20);

    spawner
        .spawn(log_button_state())
        .expect("Failed to spawn log button state task");

    spawner
        .spawn(button_task(debounced_btn_left, Direction::Left))
        .expect("Failed to spawn left button task");

    spawner
        .spawn(button_task(debounced_btn_right, Direction::Right))
        .expect("Failed to spawn right button task");
}
