use crate::display_driver::frame_buffer::{
    FrameBuffer, FrameBufferInterface, Rgb565PixelFormat, Rgb565PixelLayout,
};
use crate::display_driver::st7735_driver::command::Command;
use crate::display_driver::Encode;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

pub enum Error<SpiError, DcError, RstError> {
    Spi(SpiError),
    Dc(DcError),
    Reset(RstError),
}

pub type St7735Rgb565FrameBuffer<const WIDTH: usize, const HEIGHT: usize, const BYTES: usize> =
    FrameBuffer<WIDTH, HEIGHT, BYTES, Rgb565PixelLayout, Rgb565PixelFormat>;

// TODO: make it generic over frame buffer type
pub struct St7735<SPI, DC, RST, FB, D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    FB: FrameBufferInterface,
    D: DelayNs,
{
    device: SPI,
    dc: DC,   // low for command, high for data
    rst: RST, // active low
    frame_buffer: FB,
    delay: D,
}

impl<SPI, DC, RST, FB, D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
    St7735<SPI, DC, RST, FB, D, WIDTH, HEIGHT, BYTES>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    FB: FrameBufferInterface,
    D: DelayNs,
{
    pub fn new(device: SPI, dc: DC, rst: RST, frame_buffer: FB, delay: D) -> Self {
        Self {
            device,
            dc,
            rst,
            frame_buffer,
            delay,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    pub async fn flush(&mut self) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    pub async fn reset(&mut self) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.rst.set_low().map_err(|e| Error::Reset(e))?;
        self.delay.delay_us(100).await;
        self.rst.set_high().map_err(|e| Error::Reset(e))?;
        Ok(())
    }

    pub async fn write(
        &mut self,
        command: Command,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        let mut buffer = [0u8; 17];
        let pos: usize = command.encode(&mut buffer);
        if pos > 1 {
            self.write_command(buffer[0]).await?;
        } else {
            self.write_command(buffer[0]).await?;
            self.write_command_args(&buffer[1..pos]).await?;
        }
        Ok(())
    }

    async fn write_command(
        &mut self,
        command: u8,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.dc.set_low().map_err(|e| Error::Dc(e))?;
        self.device
            .write(&[command])
            .await
            .map_err(|e| Error::Spi(e))?;

        Ok(())
    }

    async fn write_command_args(
        &mut self,
        args: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.dc.set_high().map_err(|e| Error::Dc(e))?;
        self.device.write(args).await.map_err(|e| Error::Spi(e))?;

        Ok(())
    }

    /// This method is intended to be used from embedded graphics DrawTarget trait
    pub async fn set_pixel_color(&mut self, x: usize, y: usize, color: FB::Color) {
        self.frame_buffer.set_pixel(x, y, color);
    }
}
