use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs;

#[derive(Debug)]
pub enum Error<InputPinError, OutputPinError> {
    InputError(InputPinError),
    OutputError(OutputPinError),
}
pub struct Hc165<I, O, D>
where
    I: InputPin,
    O: OutputPin,
    D: DelayNs,
{
    shift_load: O,
    clk: O,
    data_in: I,
    delay: D,
    clock_period_us: u32,
}

impl<I, O, D> Hc165<I, O, D>
where
    I: InputPin,
    O: OutputPin,
    D: DelayNs,
{
    pub fn new(
        shift_load: O,
        mut clk: O,
        data_in: I,
        delay: D,
        clock_period_us: u32,
    ) -> Result<Self, Error<I::Error, O::Error>> {
        clk.set_low().map_err(|e| Error::OutputError(e))?;

        Ok(Self {
            shift_load,
            clk,
            data_in,
            delay,
            clock_period_us,
        })
    }

    /// This version assumes that clock inhibit is always tied to GND
    pub async fn read<const N: usize>(&mut self) -> Result<[u8; N], Error<I::Error, O::Error>> {
        let mut data: [u8; N] = [0; N];
        let half_period = self.clock_period_us / 2;

        // Load parallel inputs
        self.shift_load
            .set_low()
            .map_err(|e| Error::OutputError(e))?;
        self.delay.delay_us(half_period).await;
        self.shift_load
            .set_high()
            .map_err(|e| Error::OutputError(e))?;

        for byte in &mut data {
            for _ in 0..8 {
                // QH now contains H
                let bit_value = self.data_in.is_high().map_err(|e| Error::InputError(e))?;
                *byte = (*byte << 1) | u8::from(bit_value);

                // Rising edge shifts the next bit onto QH
                self.clk.set_high().map_err(|e| Error::OutputError(e))?;
                self.delay.delay_us(half_period).await;
                self.clk.set_low().map_err(|e| Error::OutputError(e))?;
                self.delay.delay_us(half_period).await;
            }
        }

        Ok(data)
    }
}

#[cfg(test)]
impl<I, O, D> Hc165<I, O, D>
where
    I: InputPin,
    O: OutputPin,
    D: DelayNs,
{
    pub fn free(self) -> Self {
        Hc165 {
            shift_load: self.shift_load,
            clk: self.clk,
            data_in: self.data_in,
            delay: self.delay,
            clock_period_us: self.clock_period_us,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::FakeDelay;
    use embedded_hal_mock::eh1::digital::{
        Mock as PinMock, State as PinState, Transaction as PinTransaction,
    };
    use embedded_hal_mock::eh1::MockError;
    use std::io;

    #[test]
    fn should_create_new_instance_of_hc195_and_set_clk_pin_low() {
        let shift_load_transactions: [PinTransaction; 0] = [];
        let clk_transactions = [PinTransaction::set(PinState::Low)];
        let data_in_transactions: [PinTransaction; 0] = [];
        let clock_period: u32 = 250;
        let hc165_device = get_hc165_device(
            &shift_load_transactions,
            &clk_transactions,
            &data_in_transactions,
            clock_period,
        );

        let mut hc165 = hc165_device.free();
        hc165.shift_load.done();
        hc165.clk.done();
        hc165.data_in.done();
    }

    #[tokio::test]
    async fn should_return_error_for_input_pin_error_during_read() {
        let shift_load_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];
        let clk_transactions = [PinTransaction::set(PinState::Low)];
        let data_in_transactions =
            [PinTransaction::get(PinState::High).with_error(MockError::Io(io::ErrorKind::Other))];
        let clock_period: u32 = 250;
        let mut hc165_device = get_hc165_device(
            &shift_load_transactions,
            &clk_transactions,
            &data_in_transactions,
            clock_period,
        );
        let result = hc165_device.read::<1>().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InputError(_) => {}
            _ => {
                panic!("Expected Input Error")
            }
        }

        let mut hc165 = hc165_device.free();
        hc165.shift_load.done();
        hc165.clk.done();
        hc165.data_in.done();
    }

    #[tokio::test]
    async fn should_return_error_for_output_pin_error_during_read() {
        let shift_load_transactions =
            [PinTransaction::set(PinState::Low).with_error(MockError::Io(io::ErrorKind::Other))];
        let clk_transactions = [PinTransaction::set(PinState::Low)];
        let data_in_transactions: [PinTransaction; 0] = [];
        let clock_period: u32 = 250;
        let mut hc165_device = get_hc165_device(
            &shift_load_transactions,
            &clk_transactions,
            &data_in_transactions,
            clock_period,
        );
        let result = hc165_device.read::<1>().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::OutputError(_) => {}
            _ => {
                panic!("Expected Output Error")
            }
        }

        let mut hc165 = hc165_device.free();
        hc165.shift_load.done();
        hc165.clk.done();
        hc165.data_in.done();
    }

    #[tokio::test]
    async fn should_return_available_serial_data() {
        let shift_load_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];
        let clk_transactions = [
            PinTransaction::set(PinState::Low),
            // First byte clock
            // Bit 0
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 1
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 2
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 3
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 4
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 5
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 6
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 7
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Second byte clock
            // Bit 0
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 1
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 2
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 3
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 4
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 5
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 6
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
            // Bit 7
            PinTransaction::set(PinState::High),
            PinTransaction::set(PinState::Low),
        ];
        let data_in_transactions = [
            // 0xAA
            PinTransaction::get(PinState::High),
            PinTransaction::get(PinState::Low),
            PinTransaction::get(PinState::High),
            PinTransaction::get(PinState::Low),
            PinTransaction::get(PinState::High),
            PinTransaction::get(PinState::Low),
            PinTransaction::get(PinState::High),
            PinTransaction::get(PinState::Low),
            // 0x55
            PinTransaction::get(PinState::Low),
            PinTransaction::get(PinState::High),
            PinTransaction::get(PinState::Low),
            PinTransaction::get(PinState::High),
            PinTransaction::get(PinState::Low),
            PinTransaction::get(PinState::High),
            PinTransaction::get(PinState::Low),
            PinTransaction::get(PinState::High),
        ];
        let clock_period: u32 = 1;
        let mut hc165_device = get_hc165_device(
            &shift_load_transactions,
            &clk_transactions,
            &data_in_transactions,
            clock_period,
        );
        let data = hc165_device.read::<2>().await.unwrap();
        assert_eq!(0xAA, data[0]);
        assert_eq!(0x55, data[1]);

        let mut hc165 = hc165_device.free();
        hc165.shift_load.done();
        hc165.clk.done();
        hc165.data_in.done();
    }

    fn get_hc165_device(
        shift_load_transactions: &[PinTransaction],
        clk_transactions: &[PinTransaction],
        data_in_transactions: &[PinTransaction],
        clock_period: u32,
    ) -> Hc165<PinMock, PinMock, FakeDelay> {
        let shift_load = PinMock::new(shift_load_transactions);
        let clk = PinMock::new(clk_transactions);
        let data_in = PinMock::new(data_in_transactions);
        let delay = FakeDelay::new();
        Hc165::new(shift_load, clk, data_in, delay, clock_period).unwrap()
    }
}
