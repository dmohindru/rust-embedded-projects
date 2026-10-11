use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiDevice;

pub struct St7735<SPI, DC, RST, const WIDTH: usize, const HEIGHT: usize>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
{
    device: SPI,
    dc: DC,   // low for command, high for data
    rst: RST, // active low
}
