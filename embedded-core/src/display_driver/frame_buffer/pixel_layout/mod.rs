mod rgb565_pixel_layout;
mod ssd1306_pixel_layout;

pub use rgb565_pixel_layout::Rgb565PixelLayout;
pub use ssd1306_pixel_layout::Ssd1306PixelLayout;

pub trait PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32);

    fn read_pixel(framebuffer: &[u8], width: usize, x: usize, y: usize) -> u32;
}
