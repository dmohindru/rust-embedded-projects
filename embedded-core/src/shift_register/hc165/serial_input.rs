use crate::shift_register::{ErrorHC165, Hc165, SerialInput};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs;

impl<I, O, D, const N: usize> SerialInput<N> for Hc165<I, O, D>
where
    I: InputPin,
    O: OutputPin,
    D: DelayNs,
{
    type Error = ErrorHC165<I::Error, O::Error>;

    async fn read(&mut self) -> Result<[u8; N], Self::Error> {
        Hc165::read::<N>(self).await
    }
}
