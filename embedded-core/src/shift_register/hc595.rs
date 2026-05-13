use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiDevice;

/*
Pin mapping to SPI
SRCLK   -> SCK
SER     -> MOSI
RCLK    -> CS

Handled by driver/manually
SRCLR   -> Driver handled
OE      -> Driver handled
*/

pub struct Hc595<D, P, const S: usize>
where
    D: SpiDevice,
    P: OutputPin,
{
    device: D,
    output_enable: P,
    register_clear: P,
}
