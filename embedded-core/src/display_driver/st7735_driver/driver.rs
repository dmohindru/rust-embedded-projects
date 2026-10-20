use crate::display_driver::frame_buffer::{
    FrameBuffer, FrameBufferInterface, Rgb565PixelFormat, Rgb565PixelLayout,
};
use crate::display_driver::st7735_driver::command::{ColorFormat, Command};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiDevice;

pub enum St7735Error<SpiError, DcError, RstError> {
    Spi(SpiError),
    Dc(DcError),
    Reset(RstError),
}

pub type St7735Rgb565FrameBuffer<const WIDTH: usize, const HEIGHT: usize, const BYTES: usize> =
    FrameBuffer<WIDTH, HEIGHT, BYTES, Rgb565PixelLayout, Rgb565PixelFormat>;

// TODO: make it generic over frame buffer type
pub struct St7735<SPI, DC, RST, FB, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    FB: FrameBufferInterface,
{
    device: SPI,
    dc: DC,   // low for command, high for data
    rst: RST, // active low
    frame_buffer: FB,
}

impl<SPI, DC, RST, FB, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
    St7735<SPI, DC, RST, FB, WIDTH, HEIGHT, BYTES>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    FB: FrameBufferInterface,
{
    pub fn new(device: SPI, dc: DC, rst: RST, frame_buffer: FB) -> Self {
        Self {
            device,
            dc,
            rst,
            frame_buffer,
        }
    }

    pub async fn initialize(
        &mut self,
    ) -> Result<(), St7735Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    pub async fn flush(&mut self) -> Result<(), St7735Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    pub async fn write(
        &mut self,
        command: Command,
    ) -> Result<(), St7735Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    async fn write_command(
        &mut self,
        command: u8,
    ) -> Result<(), St7735Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    async fn write_command_args(
        &mut self,
        args: &[u8],
    ) -> Result<(), St7735Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    /// This method is intended to be used from embedded graphics DrawTarget trait
    pub async fn set_pixel_color(&mut self, x: usize, y: usize, color: FB::Color) {
        self.frame_buffer.set_pixel(x, y, color);
    }
}
