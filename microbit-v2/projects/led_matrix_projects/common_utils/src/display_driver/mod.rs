mod microbit_led_driver;
pub use microbit_led_driver::MicroBitLedDriver;

// pub trait DisplayDriver {
//     /// Called once per refresh tick with the current frame data.
//     /// The implementor decides how to push it to hardware.
//     fn render(&mut self, frame: &Frame);
// }

// pub trait Delay {
//     fn delay_us(&mut self, us: u32);
// }

// pub trait PinOutput {
//     fn set_high(&mut self);
//     fn set_low(&mut self);
// }
