use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiDevice;

/*
Pin mapping to SPI
SRCLK   -> SCK
SER     -> MOSI

Handled by driver/manually
RCLK    -> Driver handled
SRCLR   -> Driver handled
OE      -> Driver handled
*/

pub enum Error<SpiError, PinError> {
    Spi(SpiError),
    Pin(PinError),
}

pub struct Hc595<SPI, PIN, const N: usize>
where
    SPI: SpiDevice,
    PIN: OutputPin,
{
    device: SPI,
    latch: PIN,
    output_enable: Option<PIN>,
    register_clear: Option<PIN>,
}

impl<SPI, PIN, const N: usize> Hc595<SPI, PIN, N>
where
    SPI: SpiDevice,
    PIN: OutputPin,
{
    pub fn new(
        device: SPI,
        mut latch: PIN,
        mut output_enable: Option<PIN>,
        mut register_clear: Option<PIN>,
    ) -> Result<Self, Error<SPI::Error, PIN::Error>> {
        latch.set_low().map_err(Error::Pin)?;
        if let Some(pin) = output_enable.as_mut() {
            pin.set_low().map_err(Error::Pin)?;
        }

        if let Some(pin) = register_clear.as_mut() {
            pin.set_high().map_err(Error::Pin)?;
        }

        Ok(Hc595 {
            device,
            latch,
            output_enable,
            register_clear,
        })
    }

    pub async fn write(&mut self, data: &[u8; N]) -> Result<(), Error<SPI::Error, PIN::Error>> {
        self.device.write(data).await.map_err(Error::Spi)?;
        self.latch.set_high().map_err(Error::Pin)?;
        self.latch.set_low().map_err(Error::Pin)?;
        Ok(())
    }

    pub fn enable(&mut self) -> Result<(), Error<SPI::Error, PIN::Error>> {
        if let Some(pin) = self.output_enable.as_mut() {
            pin.set_low().map_err(Error::Pin)?;
        }
        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), Error<SPI::Error, PIN::Error>> {
        if let Some(pin) = self.output_enable.as_mut() {
            pin.set_high().map_err(Error::Pin)?;
        }
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Error<SPI::Error, PIN::Error>> {
        if let Some(pin) = self.register_clear.as_mut() {
            pin.set_low().map_err(Error::Pin)?;
            pin.set_high().map_err(Error::Pin)?;
        }

        self.latch.set_high().map_err(Error::Pin)?;
        self.latch.set_low().map_err(Error::Pin)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_test() {}
}
