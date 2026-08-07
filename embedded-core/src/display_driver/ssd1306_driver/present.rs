use crate::{display::Present, display_driver::Ssd1306};
use embedded_hal_async::i2c::I2c;

impl<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize, const TX_BYTES: usize> Present
    for Ssd1306<D, WIDTH, HEIGHT, BYTES, TX_BYTES>
where
    D: I2c,
{
    type PresentError = D::Error;

    async fn present(&mut self) -> Result<(), Self::PresentError> {
        self.flush().await
    }
}
