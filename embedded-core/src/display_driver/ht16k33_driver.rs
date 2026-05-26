use crate::frame::Frame;
use embedded_hal_async::i2c::I2c;

pub struct Ht16K33<D, const R: usize, const C: usize>
where
    D: I2c,
{
    device: D,
    address: u8,
}

impl<D, const R: usize, const C: usize> Ht16K33<D, R, C>
where
    D: I2c,
{
    const ASSERT_DIMENSIONS: () = {
        assert!(R == 8);
        assert!(C <= 8);
    };
    pub fn new(device: D, address: u8) -> Self {
        todo!()
    }

    pub async fn initialize(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub async fn write_bitmap(&mut self, frame: &Frame<R, C>) -> Result<(), D::Error> {
        todo!()
    }
}

#[cfg(test)]
impl<D, const R: usize, const C: usize> Ht16K33<D, R, C>
where
    D: I2c,
{
    pub fn free(self) -> D {
        self.device
    }
}

/*
Start up routine
1. System setup: 0x21 → oscillator ON
2. Dimming set: 0xEF → full brightness
3. Display setup/blink set: 0x81 → display ON
*/

/*
Data writing routine
1. Display data Address pointer: 0x00
2. Write eight bytes of data
*/

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn should_initialize_device_with_proper_commands() {
        todo!()
    }

    #[tokio::test]
    async fn should_return_error_if_initialize_command_fails() {
        todo!()
    }

    #[tokio::test]
    async fn should_write_frame_data_with_proper_commands() {
        todo!()
    }

    #[tokio::test]
    async fn should_return_error_if_frame_data_write_command_fails() {
        todo!()
    }
}
