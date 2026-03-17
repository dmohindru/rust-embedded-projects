#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_time::Delay;
use embedded_core::button::DebouncedButton;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let btn_a = Input::new(p.PIN_15, Pull::Up);
    let btn_b = Input::new(p.PIN_16, Pull::Up);
    let debounced_btn_a = DebouncedButton::new(btn_a, Delay, 20);
    let debounced_btn_b = DebouncedButton::new(btn_b, Delay, 20);
}
