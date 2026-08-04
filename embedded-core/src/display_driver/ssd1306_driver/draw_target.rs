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
        todo!()
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
