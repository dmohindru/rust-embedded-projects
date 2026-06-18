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
        let _ = Self::ASSERT_DIMENSIONS;
        Self { device, address }
    }

    pub async fn initialize(&mut self) -> Result<(), D::Error> {
        // System setup oscillator on
        self.device.write(self.address, &[0x21]).await?;
        // Dimming set -> full brightness
        self.device.write(self.address, &[0xEF]).await?;
        // Display setup/blink set -> display on
        self.device.write(self.address, &[0x81]).await?;

        Ok(())
    }

    pub async fn write_bitmap(&mut self, frame: &Frame<R, C>) -> Result<(), D::Error> {
        let mut data = [0u8; 17];
        data[0] = 0x00;
        for row in 0..8 {
            data[1 + row * 2] = (*frame.get_row(row) as u8).reverse_bits();
            data[1 + row * 2 + 1] = 0;
        }
        self.device.write(self.address, &data).await
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
    use super::*;
    use embedded_hal::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    static DEVICE_ADDRESS: u8 = 0x70;

    #[tokio::test]
    async fn should_initialize_device_with_proper_commands() {
        let system_setup_command: u8 = 0x21;
        let dimming_setup_command: u8 = 0xEF;
        let display_setup_command: u8 = 0x81;
        let expectations = build_expected_write_transaction(
            vec![
                vec![system_setup_command],
                vec![dimming_setup_command],
                vec![display_setup_command],
            ],
            false,
        );
        let mut ht16k33_device = get_ht16k33_device(&expectations);

        ht16k33_device.initialize().await.unwrap();
        ht16k33_device.free().done();
    }

    #[tokio::test]
    async fn should_return_error_if_initialize_command_fails() {
        let expectations = build_expected_write_transaction(vec![vec![0x21]], true);
        let mut ht16k33_device = get_ht16k33_device(&expectations);

        let err = ht16k33_device.initialize().await.unwrap_err();
        assert_eq!(err, ErrorKind::Other);
        ht16k33_device.free().done();
    }

    #[tokio::test]
    async fn should_write_frame_data_with_proper_commands() {
        let addr_ptr_command = 0x00;
        let frame_data = [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17];
        let frame: Frame<8, 8> = Frame::new(frame_data.clone());
        let mut frame_write_transaction = Vec::<u8>::new();

        frame_write_transaction.push(addr_ptr_command);
        for data in frame_data {
            frame_write_transaction.push((data as u8).reverse_bits());
            frame_write_transaction.push(0);
        }
        let expectations = build_expected_write_transaction(vec![frame_write_transaction], false);
        let mut htc16kk33_device = get_ht16k33_device(&expectations);

        htc16kk33_device.write_bitmap(&frame).await.unwrap();
        htc16kk33_device.free().done();
    }

    #[tokio::test]
    async fn should_return_error_if_frame_data_write_command_fails() {
        let addr_ptr_command = 0x00;
        let frame_data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let frame: Frame<8, 8> = Frame::new(frame_data);
        let mut frame_write_transaction = Vec::<u8>::new();
        frame_write_transaction.push(addr_ptr_command);
        for data in frame_data {
            frame_write_transaction.push((data as u8).reverse_bits());
            frame_write_transaction.push(0);
        }
        let expectations = build_expected_write_transaction(vec![frame_write_transaction], true);
        let mut ht16k33_device = get_ht16k33_device(&expectations);

        let err = ht16k33_device.write_bitmap(&frame).await.unwrap_err();
        assert_eq!(err, ErrorKind::Other);
        ht16k33_device.free().done();
    }

    fn get_ht16k33_device(expectations: &Vec<I2cTransaction>) -> Ht16K33<I2cMock, 8, 8> {
        let i2c_device = I2cMock::new(expectations);
        let ht16k33_device = Ht16K33::<_, 8, 8>::new(i2c_device, DEVICE_ADDRESS);
        ht16k33_device
    }

    fn build_expected_write_transaction(
        transaction_data: Vec<Vec<u8>>,
        introduce_error: bool,
    ) -> Vec<I2cTransaction> {
        if introduce_error {
            transaction_data
                .iter()
                .map(|t| {
                    vec![I2cTransaction::write(DEVICE_ADDRESS, t.to_vec())
                        .with_error(ErrorKind::Other)]
                })
                .flat_map(|f| f)
                .collect()
            //vec![I2cTransaction::write(DEVICE_ADDRESS, vec![]).with_error(ErrorKind::Other)]
        } else {
            transaction_data
                .iter()
                .map(|t| vec![I2cTransaction::write(DEVICE_ADDRESS, t.to_vec())])
                .flat_map(|f| f)
                .collect()
        }
    }
}
