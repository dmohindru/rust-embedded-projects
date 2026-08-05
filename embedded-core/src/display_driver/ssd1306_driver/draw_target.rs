use crate::display_driver::Ssd1306;
use core::iter::IntoIterator;
use embedded_graphics::{
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_hal_async::i2c::I2c;

impl<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize, const TX_BYTES: usize>
    DrawTarget for Ssd1306<D, WIDTH, HEIGHT, BYTES, TX_BYTES>
where
    D: I2c,
{
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let width: i32 = WIDTH as i32;
        let height: i32 = HEIGHT as i32;
        for Pixel(coord, color) in pixels {
            let x = coord.x;
            let y = coord.y;

            if x >= 0 && x < width && y >= 0 && y < height {
                match color {
                    BinaryColor::On => self.set_pixel(x as usize, y as usize),
                    BinaryColor::Off => self.clear_pixel(x as usize, y as usize),
                }
            }
        }
        Ok(())
    }
}

impl<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize, const TX_BYTES: usize>
    OriginDimensions for Ssd1306<D, WIDTH, HEIGHT, BYTES, TX_BYTES>
where
    D: I2c,
{
    fn size(&self) -> Size {
        Size::new(128, 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_driver::Ssd1306_128x64;
    use embedded_hal_mock::eh1::i2c::Mock as I2cMock;

    #[test]
    fn should_set_pixels_in_display() {
        let i2c_device = I2cMock::new(vec![]);
        let display = Ssd1306_128x64::<_>::new(i2c_device, 0x00);

        let (_, frame_buffer) = display.free();
    }
}
