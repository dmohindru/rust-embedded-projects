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
    Latch(PinError),
    OutputEnable(PinError),
    RegisterClear(PinError),
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
        // Establish driver invariants
        latch.set_low().map_err(Error::Latch)?;

        if let Some(pin) = output_enable.as_mut() {
            // OE is active LOW
            pin.set_low().map_err(Error::OutputEnable)?;
        }

        if let Some(pin) = register_clear.as_mut() {
            // SRCLR is active LOW
            pin.set_high().map_err(Error::RegisterClear)?;
        }

        Ok(Self {
            device,
            latch,
            output_enable,
            register_clear,
        })
    }

    pub async fn write(&mut self, data: &[u8; N]) -> Result<(), Error<SPI::Error, PIN::Error>> {
        self.device.write(data).await.map_err(Error::Spi)?;

        self.latch.set_high().map_err(Error::Latch)?;
        self.latch.set_low().map_err(Error::Latch)?;

        Ok(())
    }

    pub fn enable(&mut self) -> Result<(), Error<SPI::Error, PIN::Error>> {
        if let Some(pin) = self.output_enable.as_mut() {
            pin.set_low().map_err(Error::OutputEnable)?;
        }

        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), Error<SPI::Error, PIN::Error>> {
        if let Some(pin) = self.output_enable.as_mut() {
            pin.set_high().map_err(Error::OutputEnable)?;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Error<SPI::Error, PIN::Error>> {
        if let Some(pin) = self.register_clear.as_mut() {
            pin.set_low().map_err(Error::RegisterClear)?;
            pin.set_high().map_err(Error::RegisterClear)?;
        }

        // Propagate cleared shift register to outputs
        self.latch.set_high().map_err(Error::Latch)?;
        self.latch.set_low().map_err(Error::Latch)?;

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
    use std::io;

    use super::*;
    use embedded_hal_mock::common::Generic;
    use embedded_hal_mock::eh1::digital::{
        Mock as PinMock, State as PinState, Transaction as PinTransaction,
    };
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
    use embedded_hal_mock::eh1::MockError;

    fn setup_hc595(
        spi_transactions: &[SpiTransaction<u8>],
        latch_pin_transactions: &[PinTransaction],
        oe_pin_transactions: Option<&[PinTransaction]>,
        rc_pin_transactions: Option<&[PinTransaction]>,
    ) -> Hc595<Generic<SpiTransaction<u8>>, embedded_hal_mock::eh1::digital::Mock, 2> {
        let latch_pin = PinMock::new(latch_pin_transactions);
        let output_enable_pin = match oe_pin_transactions {
            Some(transactions) => Some(PinMock::new(transactions)),
            _ => None,
        };
        let register_clear_pin = match rc_pin_transactions {
            Some(transactions) => Some(PinMock::new(transactions)),
            _ => None,
        };
        let spi: Generic<SpiTransaction<u8>> = SpiMock::new(spi_transactions);

        Hc595::<_, _, 2>::new(spi, latch_pin, output_enable_pin, register_clear_pin).unwrap()
    }

    #[test]
    fn should_create_new_instance_initialize_properly() {
        let latch_pin_transactions = [PinTransaction::set(PinState::Low)];
        let output_pin_transactions = [PinTransaction::set(PinState::Low)];
        let register_clear_pin_transactions = [PinTransaction::set(PinState::High)];
        let spi_transactions: &[SpiTransaction<u8>] = &[];

        let hc595 = setup_hc595(
            &spi_transactions,
            &latch_pin_transactions,
            Some(&output_pin_transactions),
            Some(&register_clear_pin_transactions),
        );
        let mut hc595 = hc595.free();

        hc595.device.done();
        hc595.latch.done();
        hc595.output_enable.unwrap().done();
        hc595.register_clear.unwrap().done();
    }

    #[tokio::test]
    async fn should_write_data_hc595_chip() {
        let latch_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
        ];
        let spi_transactions = [
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x01_u8, 0x02_u8]),
            SpiTransaction::transaction_end(),
        ];

        let mut hc595 = setup_hc595(&spi_transactions, &latch_pin_transactions, None, None);
        let result = hc595.write(&[0x01_u8, 0x02_u8]).await;
        assert!(result.is_ok());
        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
    }

    #[test]
    fn should_return_error_for_spi_error_during_write_ops() {
        /*
        TODO: figure out how to simulate spi write error
        */
    }

    #[tokio::test]
    async fn should_return_error_for_pin_error_during_write_ops() {
        let latch_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High).with_error(MockError::Io(io::ErrorKind::Other)),
        ];

        let spi_transactions = [
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x01_u8, 0x02_u8]),
            SpiTransaction::transaction_end(),
        ];

        let mut hc595 = setup_hc595(&spi_transactions, &latch_pin_transactions, None, None);
        let result = hc595.write(&[0x01_u8, 0x02_u8]).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            Error::Latch(_) => {}
            _ => {
                panic!("Expected LatchError")
            }
        }
        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
    }

    #[test]
    fn should_enable_output_for_hc595_chip() {
        let latch_pin_transactions = [PinTransaction::set(PinState::Low)];
        let spi_transactions: &[SpiTransaction<u8>] = &[];
        let output_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::Low),
        ];

        let mut hc595 = setup_hc595(
            spi_transactions,
            &latch_pin_transactions,
            Some(&output_pin_transactions),
            None,
        );

        let result = hc595.enable();
        assert!(result.is_ok());
        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
        hc595.output_enable.unwrap().done();
    }

    #[test]
    fn should_return_error_for_enable_ops_for_output_enable_pin_error() {
        let latch_pin_transactions = [PinTransaction::set(PinState::Low)];
        let oe_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::Low).with_error(MockError::Io(io::ErrorKind::Other)),
        ];
        let spi_transactions: &[SpiTransaction<u8>] = &[];

        let mut hc595 = setup_hc595(
            &spi_transactions,
            &latch_pin_transactions,
            Some(&oe_pin_transactions),
            None,
        );

        let result = hc595.enable();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::OutputEnable(_) => {}
            _ => {
                panic!("Expected OutputEnable error")
            }
        }

        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
        hc595.output_enable.unwrap().done();
    }

    #[test]
    fn should_disable_output_for_hc595_chip() {
        let latch_pin_transactions = [PinTransaction::set(PinState::Low)];
        let output_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];
        let spi_transactions: &[SpiTransaction<u8>] = &[];

        let mut hc595 = setup_hc595(
            spi_transactions,
            &latch_pin_transactions,
            Some(&output_pin_transactions),
            None,
        );

        let result = hc595.disable();
        assert!(result.is_ok());
        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
        hc595.output_enable.unwrap().done();
    }

    #[test]
    fn should_return_error_for_disable_ops_for_output_enable_pin_error() {
        let latch_pin_transactions = [PinTransaction::set(PinState::Low)];
        let oe_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High).with_error(MockError::Io(io::ErrorKind::Other)),
        ];
        let spi_transactions: &[SpiTransaction<u8>] = &[];

        let mut hc595 = setup_hc595(
            &spi_transactions,
            &latch_pin_transactions,
            Some(&oe_pin_transactions),
            None,
        );

        let result = hc595.disable();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::OutputEnable(_) => {}
            _ => {
                panic!("Expected OutputEnable error")
            }
        }

        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
        hc595.output_enable.unwrap().done();
    }

    #[test]
    fn should_clear_output_for_hc595_chip() {
        let latch_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
        ];

        let spi_transactions: &[SpiTransaction<u8>] = &[];
        let register_clear_transactions = [
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];

        let mut hc595 = setup_hc595(
            &spi_transactions,
            &latch_pin_transactions,
            None,
            Some(&register_clear_transactions),
        );

        let result = hc595.clear();
        assert!(result.is_ok());
        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
        hc595.register_clear.unwrap().done();
    }

    #[test]
    fn should_return_error_for_clear_ops_for_register_clear_pin_error() {
        let latch_pin_transactions = [PinTransaction::set(PinState::Low)];

        let spi_transactions: &[SpiTransaction<u8>] = &[];
        let register_clear_transactions = [
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low).with_error(MockError::Io(std::io::ErrorKind::Other)),
        ];

        let mut hc595 = setup_hc595(
            &spi_transactions,
            &latch_pin_transactions,
            None,
            Some(&register_clear_transactions),
        );

        let result = hc595.clear();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::RegisterClear(_) => {}
            _ => panic!("Expected RegisterClear error"),
        }

        let mut hc595 = hc595.free();
        hc595.device.done();
        hc595.latch.done();
        hc595.register_clear.unwrap().done();
    }
}
