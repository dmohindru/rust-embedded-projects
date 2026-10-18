mod ssd1306_pixel_layout;

pub use ssd1306_pixel_layout::Ssd1306PixelLayout;

pub trait PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32);

    fn read_pixel(framebuffer: &[u8], width: usize, x: usize, y: usize) -> u32;
}
