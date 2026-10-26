use embedded_hal::digital::{ErrorType, OutputPin};

/// An output pin whose operations have no effect.
///
/// Useful when a hardware signal is controlled externally,
/// rather than by software.
pub struct NoopOutputPin;

impl ErrorType for NoopOutputPin {
    type Error = core::convert::Infallible;
}

impl OutputPin for NoopOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
