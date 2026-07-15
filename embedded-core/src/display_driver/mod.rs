mod frame_buffer;
mod ht16k33_driver;
mod led_matrix_driver;
mod max7219_driver;
mod ssd1306_driver;
pub use ht16k33_driver::Ht16K33;
pub use led_matrix_driver::LedMatrixDriver;
pub use max7219_driver::Max7219;

trait Encode {
    fn encode(&self, out: &mut [u8]) -> usize;
}
