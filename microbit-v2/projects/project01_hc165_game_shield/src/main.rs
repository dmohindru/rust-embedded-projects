#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_time::{Delay, Timer};
use embedded_core::shift_register::Hc165;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    // Arcade Shield INSR_LATCH
    // micro:bit edge connector P9
    // nRF52833 P0.09
    let shift_load = Output::new(p.P0_09, Level::High, OutputDrive::Standard);
    // Arcade Shield SR_CLK
    // micro:bit edge connector P20
    // nRF52833 P1.00
    let clk = Output::new(p.P1_00, Level::High, OutputDrive::Standard);
    // Arcade Shield INSR0_DATA
    // micro:bit edge connector P14
    // nRF52833 P0.01
    let data_in = Input::new(p.P0_01, Pull::Down);

    let mut hc165_device = Hc165::new(shift_load, clk, data_in, Delay, 4).unwrap();

    loop {
        let data = hc165_device.read::<2>().await.unwrap();
        info!("Button Data: {=u8:02x}", data[0]);
        info!("Config Data: {=u8:02x}", data[1]);
        Timer::after_millis(200).await;
    }
}
