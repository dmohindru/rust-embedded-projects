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

#[derive(Debug)]
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
impl<SPI, PIN, const N: usize> Hc595<SPI, PIN, N>
where
    SPI: SpiDevice,
    PIN: OutputPin,
{
    pub fn free(self) -> Self {
        Hc595 {
            device: self.device,
            latch: self.latch,
            output_enable: self.output_enable,
            register_clear: self.register_clear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::common::Generic;
    use embedded_hal_mock::eh1::digital::{
        Mock as PinMock, State as PinState, Transaction as PinTransaction,
    };
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
    use embedded_hal_mock::eh1::MockError;
    use std::io::ErrorKind;

    #[test]
    fn should_create_new_instance_initialize_properly() {
        let latch_pin = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let output_enable_pin = PinMock::new(&[PinTransaction::set(PinState::Low)]);
        let register_clear_pin = PinMock::new(&[PinTransaction::set(PinState::High)]);
        let spi: Generic<SpiTransaction<u8>> = SpiMock::new(&[]);

        let hc595 = Hc595::<_, _, 1>::new(
            spi,
            latch_pin,
            Some(output_enable_pin),
            Some(register_clear_pin),
        )
        .unwrap();

        let mut hc595 = hc595.free();

        hc595.device.done();
        hc595.latch.done();
        hc595.output_enable.unwrap().done();
        hc595.register_clear.unwrap().done();
    }

    #[test]
    fn should_return_error_during_initialize_for_latch_pin_error() {
        let latch_pin = PinMock::new(&[
            PinTransaction::set(PinState::Low).with_error(MockError::Io(ErrorKind::Other))
        ]);
        let spi: Generic<SpiTransaction<u8>> = SpiMock::new(&[]);
        let hc595 = Hc595::<_, _, 1>::new(spi, latch_pin, None, None);
        assert!(hc595.is_err());
    }

    #[test]
    fn should_return_error_during_initialize_for_output_enable_pin_error() {
        todo!()
    }

    #[test]
    fn should_return_error_during_initialize_for_register_clear_pin_error() {
        todo!()
    }

    #[test]
    fn should_write_data_hc595_chip() {
        todo!()
    }

    #[test]
    fn should_return_error_for_spi_error_during_write_ops() {
        todo!()
    }

    #[test]
    fn should_return_error_for_pin_error_during_write_ops() {
        todo!()
    }

    #[test]
    fn should_enable_output_for_hc595_chip() {
        todo!()
    }

    #[test]
    fn should_return_error_for_enable_ops_for_output_enable_pin_error() {
        todo!()
    }

    #[test]
    fn should_disable_output_for_hc595_chip() {
        todo!()
    }

    #[test]
    fn should_return_error_for_disable_ops_for_output_enable_pin_error() {
        todo!()
    }

    #[test]
    fn should_clear_output_for_hc595_chip() {
        todo!()
    }

    #[test]
    fn should_return_error_for_clear_ops_for_register_clear_pin_error() {
        todo!()
    }
}
