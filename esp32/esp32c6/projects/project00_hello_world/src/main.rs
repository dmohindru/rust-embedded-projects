#![no_std]
#![no_main]

use defmt::println;
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

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // =========== Initialization ===============
    let config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = init(config);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    start(timg0.timer0, sw_interrupt.software_interrupt0);
    println!("Embassy initialized!");

    // =========== Application ===============
    loop {
        println!("Hello from ESP32C6 Chip");
        Timer::after_secs(1).await;
    }
}
