#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    mutex::{Mutex, MutexGuard},
};
use embassy_time::{Delay, Timer};
use embedded_core::{button::DebouncedButton, frame::Direction};

use esp_backtrace as _;
use esp_println as _;

use esp_hal::{
    Config,
    clock::CpuClock,
    gpio::{Input, InputConfig, Pull},
    init,
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use esp_rtos::start;

extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();

type ButtonStateType = Mutex<CriticalSectionRawMutex, Option<Direction>>;
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
            let button_opt: MutexGuard<'_, CriticalSectionRawMutex, Option<Direction>> =
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
                let mut button_state: MutexGuard<'_, CriticalSectionRawMutex, Option<Direction>> =
                    BUTTON_STATE.lock().await;
                *button_state = Some(direction);
                info!("Moving state to {}", direction)
            }
        })
        .await;
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // =========== Initialization ===============
    let config = Config::default().with_cpu_clock(CpuClock::max());
    let p = init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    let timg0 = TimerGroup::new(p.TIMG0);
    let sw_interrupt = SoftwareInterruptControl::new(p.SW_INTERRUPT);
    start(timg0.timer0, sw_interrupt.software_interrupt0);
    info!("Embassy initialized!");

    // =========== Application ===============
    // 4 -> left
    // 5 -> right
    let btn_left_config = InputConfig::default();
    let btn_left = Input::new(p.GPIO4, btn_left_config.with_pull(Pull::Up));

    let btn_right_config = InputConfig::default();
    let btn_right = Input::new(p.GPIO5, btn_right_config.with_pull(Pull::Up));
    let debounced_btn_left = DebouncedButton::new(btn_left, Delay, 20);
    let debounced_btn_right = DebouncedButton::new(btn_right, Delay, 20);

    spawner.spawn(log_button_state_task().unwrap());
    spawner.spawn(button_task(debounced_btn_left, Direction::Left).unwrap());
    spawner.spawn(button_task(debounced_btn_right, Direction::Right).unwrap());
}
