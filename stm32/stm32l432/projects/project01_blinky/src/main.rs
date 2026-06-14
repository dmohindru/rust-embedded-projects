#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config,
    gpio::{Level, Output, Speed},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PB3, Level::Low, Speed::Medium);

    loop {
        info!("led on!");
        led.set_high();
        Timer::after_secs(1).await;

        info!("led off!");
        led.set_low();
        Timer::after_secs(1).await;
    }
}
