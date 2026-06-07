#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;

use esp_backtrace as _;
use esp_println as _;

use esp_hal::{
    Config, clock::CpuClock, init, interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use esp_rtos::start;

extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn run() {
    loop {
        info!("Hello from embassy task!");
        Timer::after_millis(500).await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // =========== Initialization ===============
    let config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    start(timg0.timer0, sw_interrupt.software_interrupt0);
    info!("Embassy initialized!");

    // =========== Application ===============
    spawner.spawn(run().unwrap());
    loop {
        info!("Hello from embassy main!");
        Timer::after_secs(1).await;
    }
}
