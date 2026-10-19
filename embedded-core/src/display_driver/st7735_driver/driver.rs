use crate::display_driver::frame_buffer::{FrameBuffer, Rgb565PixelFormat, Rgb565PixelLayout};
use crate::display_driver::st7735_driver::command::{ColorFormat, Command};
use embedded_graphics::pixelcolor::Rgb565;
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
pub struct St7735<SPI, DC, RST, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
{
    device: SPI,
    dc: DC,   // low for command, high for data
    rst: RST, // active low
    frame_buffer: St7735Rgb565FrameBuffer<WIDTH, HEIGHT, BYTES>,
}

impl<SPI, DC, RST, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
    St7735<SPI, DC, RST, WIDTH, HEIGHT, BYTES>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
{
    pub fn new(device: SPI, dc: DC, rst: RST) -> Self {
        Self {
            device,
            dc,
            rst,
            frame_buffer: FrameBuffer::new(),
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

    /// This method is intended to be used from embedded graphics DrawTarget trait
    pub async fn set_pixel_color(&mut self, x: usize, y: usize, color: Rgb565) {
        todo!()
    }
}
