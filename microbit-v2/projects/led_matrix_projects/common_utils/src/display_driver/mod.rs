mod microbit_led_driver;
use alloc::boxed::Box;
use async_trait::async_trait;
pub use microbit_led_driver::EmbassyDelay;
pub use microbit_led_driver::MicroBitLedDriver;

#[async_trait]
pub trait AsyncDelay {
    async fn delay_micros(&mut self, us: u64);
}

pub trait OutputPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
}
