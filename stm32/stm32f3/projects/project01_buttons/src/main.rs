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
use embedded_core::{
    frame::Direction,
    input_device::button::{ActiveLevel, DebouncedButton},
};
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

#[embassy_executor::task]
async fn button_task(mut button: DebouncedButton<ExtiInput<'static>, Delay>) {
    button
        .wait(|| async {
            let mut button_state: MutexGuard<'_, ThreadModeRawMutex, Direction> =
                BUTTON_STATE.lock().await;
            *button_state = (*button_state).toggle();
            info!("Moving state to {}", *button_state);
        })
        .await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    let btn = ExtiInput::new(p.PA0, p.EXTI0, Pull::None);
    let debounced_btn = DebouncedButton::new(btn, Delay, 20).with_active_level(ActiveLevel::High);

    spawner
        .spawn(log_button_state())
        .expect("Failed to spawn log button state task");

    spawner
        .spawn(button_task(debounced_btn))
        .expect("Failed to spawn button task");
}
