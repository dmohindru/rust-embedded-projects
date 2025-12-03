mod microbit_driver;
use crate::frame::Frame;
use async_trait::async_trait;

#[async_trait]
pub trait DisplayDriver {
    /// Called once per refresh tick with the current frame data.
    /// The implementor decides how to push it to hardware.
    async fn render(&mut self, frame: &Frame);
}

#[async_trait]
pub trait Delay {
    async fn delay_us(&mut self, us: u32);
}

pub trait PinOutput {
    fn set_high(&mut self);
    fn set_low(&mut self);
}
