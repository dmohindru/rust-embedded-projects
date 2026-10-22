use crate::display_driver::frame_buffer::{
    FrameBuffer, FrameBufferInterface, Rgb565PixelFormat, Rgb565PixelLayout,
};
use crate::display_driver::st7735_driver::command::{ColorFormat, Command};
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

    pub async fn initialize(
        &mut self,
        color_format: ColorFormat,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.rst.set_high().map_err(|e| Error::Reset(e))?;
        self.write(Command::SWRESET).await?;
        self.write(Command::SLPOUT).await?;
        self.write(Command::FRMCTR1([0x01, 0x2C, 0x2D])).await?;
        self.write(Command::FRMCTR2([0x01, 0x2C, 0x2D])).await?;
        self.write(Command::FRMCTR3([0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D]))
            .await?;
        self.write(Command::INVCTR(0x07)).await?;
        self.write(Command::PWCTR1([0xA2, 0x02, 0x84])).await?;
        self.write(Command::PWCTR2(0xC5)).await?;
        self.write(Command::PWCTR3([0x0A, 0x00])).await?;
        self.write(Command::PWCTR4([0x8A, 0x2A])).await?;
        self.write(Command::PWCTR5([0x8A, 0xEE])).await?;
        self.write(Command::VMCTR1(0x0E)).await?;
        self.write(Command::MADCTL(0x00)).await?;
        self.write(Command::COLMOD(color_format)).await?;
        self.write(Command::GAMCTRP1([
            0x12, 0x1C, 0x10, 0x18, 0x33, 0x2C, 0x25, 0x28, 0x28, 0x27, 0x2F, 0x3C, 0x00, 0x03,
            0x03, 0x10,
        ]))
        .await?;
        self.write(Command::GAMCTRN1([
            0x12, 0x1C, 0x10, 0x18, 0x2D, 0x28, 0x23, 0x28, 0x28, 0x26, 0x2F, 0x3B, 0x00, 0x03,
            0x03, 0x10,
        ]))
        .await?;
        self.write(Command::DISPON).await?;
        self.write(Command::NORON).await?;

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::FakeDelay;

    use embedded_hal_mock::eh1::digital::{
        Mock as PinMock, State as PinState, Transaction as PinTransaction,
    };

    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
    const WIDTH: usize = 162;
    const HEIGHT: usize = 132;
    const BYTES: usize = 42768; // 162 X 132 X 2 = 42768
    type RGB565FrameBuffer = St7735Rgb565FrameBuffer<WIDTH, HEIGHT, BYTES>;

    fn get_st7735_device(
        rst_pin_transactions: &[PinTransaction],
        dc_pin_transactions: &[PinTransaction],
        spi_transactions: Vec<SpiTransaction<u8>>,
    ) -> St7735<SpiMock<u8>, PinMock, PinMock, RGB565FrameBuffer, FakeDelay, WIDTH, HEIGHT, BYTES>
    {
        let rst_pin = PinMock::new(rst_pin_transactions);
        let dc_pin = PinMock::new(dc_pin_transactions);
        let spi_device = SpiMock::new(&spi_transactions);
        let fake_delay = FakeDelay::new();
        let frame_buffer = RGB565FrameBuffer::new();
        let st7735_device = St7735::new(spi_device, dc_pin, rst_pin, frame_buffer, fake_delay);
        st7735_device
    }
}
