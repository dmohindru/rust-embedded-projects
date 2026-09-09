#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config,
    gpio::{Input, Level, Output, Pull, Speed},
};
use embassy_time::{Delay, Timer};
use embedded_core::shift_register::Hc165;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    // PA0 -> CLK (Output)
    let clk = Output::new(p.PA0, Level::Low, Speed::Medium);
    // PA1 -> SH/LD (Output)
    let shift_load = Output::new(p.PA1, Level::High, Speed::Medium);
    // PA3 -> Qh (Input)
    let data_in = Input::new(p.PA3, Pull::Down);

    let mut hc165_device = Hc165::new(shift_load, clk, data_in, Delay, 4).unwrap();

    loop {
        Timer::after_millis(500).await;
        let data = hc165_device.read::<1>().await.unwrap();
        info!("Data: {=u8:02x}", data[0]);
    }
}
