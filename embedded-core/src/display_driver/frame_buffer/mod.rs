mod frame_buffer;
mod pixel_format;
mod pixel_layout;
use embedded_graphics::pixelcolor::PixelColor;
pub use frame_buffer::FrameBuffer;
pub use pixel_format::{BinaryPixelFormat, PixelFormat, Rgb565PixelFormat};
pub use pixel_layout::{PixelLayout, Rgb565PixelLayout, Ssd1306PixelLayout};

pub trait FrameBufferInterface {
    type Color: PixelColor;

    fn set_pixel(&mut self, x: usize, y: usize, color: Self::Color);

    fn frame_data(&self) -> &[u8];

    fn clear_frame_data(&mut self);

    fn width(&self) -> usize;

    fn height(&self) -> usize;
}
