use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs;
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
    clk: O,
    clk_inhabit: Option<O>,
    data_in: I,
    delay: D,
    clk_period_ms: u32,
}

impl<I, O, D> Hc165<I, O, D>
where
    I: InputPin,
    O: OutputPin,
    D: DelayNs,
{
    pub fn new(clk: O, clk_inhabit: Option<O>, data_in: I, delay: D, clk_period_ms: u32) -> Self {
        Self {
            clk,
            clk_inhabit,
            data_in,
            delay,
            clk_period_ms,
        }
    }

    pub async fn read<const N: usize>(
        &mut self,
        bytes: usize,
    ) -> Result<[u8; N], Error<I::Error, O::Error>> {
        let mut data: [u8; N] = [0; N];
        if let Some(pin) = self.clk_inhabit.as_mut() {
            pin.set_low().map_err(|e| Error::OutputError(e))?;
        }

        for _ in 0..(bytes * 8) {
            self.clk.set_low().map_err(|e| Error::OutputError(e))?;
            self.delay.delay_us(self.clk_period_ms / 2).await;
            self.clk.set_high().map_err(|e| Error::OutputError(e))?;
            let high_result = self.data_in.is_high().map_err(|e| Error::InputError(e))?;
            let low_result = self.data_in.is_low().map_err(|e| Error::InputError(e))?;
            let bit: u8 = if high_result && !low_result { 1 } else { 0 };
        }

        Ok(data)
    }
}
