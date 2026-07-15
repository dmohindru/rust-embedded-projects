use crate::display_driver::frame_buffer::{BinaryPixelFormat, FrameBuffer, Ssd1306PixelLayout};
use embedded_hal_async::i2c::I2c;

pub struct Ssd1306<D>
where
    D: I2c,
{
    device: D,
    address: u8,
    // TODO Generalize over various screen sizes
    frame_buffer: FrameBuffer<128, 64, 1024, Ssd1306PixelLayout, BinaryPixelFormat>,
}

impl<D> Ssd1306<D>
where
    D: I2c,
{
    pub fn new(device: D, address: u8) -> Self {
        Self {
            device,
            address,
            frame_buffer: FrameBuffer::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub fn flush(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub fn write_commands(&mut self, commands: &[u8]) -> Result<(), D::Error> {
        todo!()
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<(), D::Error> {
        todo!()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        todo!()
    }

    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        todo!()
    }
}
