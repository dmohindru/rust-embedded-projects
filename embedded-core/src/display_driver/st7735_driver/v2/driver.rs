use crate::display_driver::st7735_driver::{ColorFormat, Error};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, spi::SpiDevice};

pub struct St7735<SPI, DC, RST, D, const WIDTH: usize, const HEIGHT: usize>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    D: DelayNs,
{
    device: SPI,
    dc: DC,
    rst: RST,
    delay: D,
    color_format: ColorFormat,
}

impl<SPI, DC, RST, D, const WIDTH: usize, const HEIGHT: usize>
    St7735<SPI, DC, RST, D, WIDTH, HEIGHT>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    D: DelayNs,
{
    pub fn new(device: SPI, dc: DC, rst: RST, delay: D) -> Self {
        Self {
            device,
            dc,
            rst,
            delay,
            color_format: ColorFormat::SixteenBPP,
        }
    }

    pub fn color_format(mut self, color_format: ColorFormat) -> Self {
        self.color_format = color_format;
        self
    }

    pub async fn initialize(&mut self) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    // Validation	Requirement
    // Width	Greater than zero
    // Height	Greater than zero
    // Coordinates	Rectangle must fit within display dimensions
    // Pixel data	Exactly width × height × 2 bytes for tightly packed data
    // Stride	At least width × 2 bytes
    // Strided data	At least (height - 1) × stride + width × 2 bytes
    // Stride = Width X BytesPerPixel

    pub async fn update_rect(
        &mut self,
        x: u16,
        y: u16,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    pub async fn update_rect_stride(
        &mut self,
        x: u16,
        y: u16,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    async fn set_address_window(
        &mut self,
        x: u16,
        y: u16,
        width: usize,
        height: usize,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        // Validate region and data length
        // Configure address window
        // Issue RAMWR
        // Transfer pixel data
        todo!()
    }

    async fn write_pixel_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        // Validate region, stride and data length
        // Configure address window
        // Issue RAMWR
        // Transfer each row, respecting stride
        todo!()
    }
}

#[cfg(test)]
impl<SPI, DC, RST, D, const WIDTH: usize, const HEIGHT: usize>
    St7735<SPI, DC, RST, D, WIDTH, HEIGHT>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    D: DelayNs,
{
    pub fn free(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {}
}
