#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{Config, clock::CpuClock, init, interrupt::software::SoftwareInterruptControl};
use esp_rtos::start;
use rtt_target::rprintln;

extern crate alloc;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", panic_info);
    loop {}
}
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // =========== Initialization ===============
    rtt_target::rtt_init_print!();

    let config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    start(timg0.timer0, sw_interrupt.software_interrupt0);
    rprintln!("Embassy initialized!");

    // =========== Application ===============
    loop {
        rprintln!("Hello from ESP32C6 Chip");
        Timer::after_secs(1).await;
    }
}
