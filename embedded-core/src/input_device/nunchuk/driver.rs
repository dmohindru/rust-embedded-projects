use crate::input_device::{nunchuk::command::Command, Encode};
use embedded_hal_async::i2c::I2c;

pub enum NunchukType {
    White,
    Black,
}

pub struct NunchukReport {
    pub x_axis: u8,
    pub y_axis: u8,
    pub x_acceleration: u16,
    pub y_acceleration: u16,
    pub z_acceleration: u16,
    pub c_button_pressed: bool,
    pub z_button_pressed: bool,
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

    pub async fn poll(&mut self) -> Result<NunchukReport, D::Error> {
        let mut data: [u8; 6] = [0; 6];
        let mask: u8 = 0x03;
        self.device.read(self.address, &mut data).await?;
        let x_acceleration: u16 = (data[2] << 2) | (data[5] >> 2 & mask);
        let y_acceleration: u16 = (data[3] << 2) | (data[5] >> 4 & mask);
        let z_acceleration: u16 = (data[4] << 2) | (data[5] >> 6 & mask);
        let c_button_pressed = if data[5] & 0x01 == 0 { true } else { false };
        let z_button_pressed = if (data[5] >> 1) & 0x01 == 0 {
            true
        } else {
            false
        };
        let nunchuk_report = NunchukReport {
            x_axis: data[0],
            y_axis: data[1],
            x_acceleration,
            y_acceleration,
            z_acceleration,
            c_button_pressed,
            z_button_pressed,
        };
        Ok(nunchuk_report)
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
