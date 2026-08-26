use crate::input_device::{nunchuk::command::Command, Encode};
use embedded_hal_async::i2c::I2c;

pub enum NunchukType {
    White,
    Black,
}

pub struct Nunchuk<D>
where
    D: I2c,
{
    device: D,
    address: u8,
    nunchuk_type: NunchukType,
}

impl<D> Nunchuk<D>
where
    D: I2c,
{
    pub fn new(device: D, address: u8, nunchuk_type: NunchukType) -> Self {
        Self {
            device,
            address,
            nunchuk_type,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), D::Error> {
        let mut buffer = [0u8; 4];
        let mut len: usize;
        match self.nunchuk_type {
            NunchukType::Black => {
                len = Command::BlackInitFirstRegister.encode(&mut buffer);
                self.device.write(self.address, &buffer[..len]).await?;
                len = Command::BlackInitSecondRegister.encode(&mut buffer);
                self.device.write(self.address, &buffer[..len]).await?;
            }
            NunchukType::White => {
                len = Command::WhiteInitFirstRegister.encode(&mut buffer);
                self.device.write(self.address, &buffer[..len]).await?;
                len = Command::WhiteInitSecondRegister.encode(&mut buffer);
                self.device.write(self.address, &buffer[..len]).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl<D> Nunchuk<D>
where
    D: I2c,
{
    pub fn free(self) -> D {
        self.device
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    static DEVICE_ADDRESS: u8 = 0x52;

    #[tokio::test]
    async fn should_initialize_black_nunchuk_with_proper_commands() {
        let first_register_bytes: Vec<u8> = Vec::from([0xF0, 0x55]);
        let first_register_expected_commands =
            I2cTransaction::write(DEVICE_ADDRESS, first_register_bytes);

        let second_register_bytes: Vec<u8> = Vec::from([0xFB, 0x00]);
        let second_register_expected_commands =
            I2cTransaction::write(DEVICE_ADDRESS, second_register_bytes);
        let mut nunchuk_device = get_nunchuk_device(
            NunchukType::Black,
            &vec![
                first_register_expected_commands,
                second_register_expected_commands,
            ],
        );
        nunchuk_device.initialize().await.unwrap();
        nunchuk_device.free().done();
    }

    #[tokio::test]
    async fn should_initialize_white_nunchuk_with_proper_commands() {
        let first_register_bytes: Vec<u8> = Vec::from([0x40, 0x00]);
        let first_register_expected_commands =
            I2cTransaction::write(DEVICE_ADDRESS, first_register_bytes);

        let second_register_bytes: Vec<u8> = Vec::from([0x00]);
        let second_register_expected_commands =
            I2cTransaction::write(DEVICE_ADDRESS, second_register_bytes);
        let mut nunchuk_device = get_nunchuk_device(
            NunchukType::White,
            &vec![
                first_register_expected_commands,
                second_register_expected_commands,
            ],
        );
        nunchuk_device.initialize().await.unwrap();
        nunchuk_device.free().done();
    }

    fn get_nunchuk_device(
        nunchuk_type: NunchukType,
        expectations: &Vec<I2cTransaction>,
    ) -> Nunchuk<I2cMock> {
        let i2c_device = I2cMock::new(expectations);
        Nunchuk::<_>::new(i2c_device, DEVICE_ADDRESS, nunchuk_type)
    }
}
