use crate::display_driver::frame_buffer::{BinaryPixelFormat, FrameBuffer, Ssd1306PixelLayout};
use embedded_hal_async::i2c::I2c;

pub struct Ssd1306<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
where
    D: I2c,
{
    device: D,
    address: u8,
    // TODO Generalize over various screen sizes
    frame_buffer: FrameBuffer<WIDTH, HEIGHT, BYTES, Ssd1306PixelLayout, BinaryPixelFormat>,
}

impl<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
    Ssd1306<D, WIDTH, HEIGHT, BYTES>
where
    D: I2c,
{
    pub fn new(device: D, address: u8) -> Self {
        Self {
            device,
            address,
            frame_buffer: FrameBuffer::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub fn flush(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub fn write_commands(&mut self, commands: &[u8]) -> Result<(), D::Error> {
        todo!()
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<(), D::Error> {
        todo!()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        todo!()
    }

    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        todo!()
    }
}

#[cfg(test)]
impl<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize>
    Ssd1306<D, WIDTH, HEIGHT, BYTES>
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
    use crate::display_driver::{ssd1306_driver::command::Command, Encode};
    use embedded_hal::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    static DEVICE_ADDRESS: u8 = 0x3D;

    #[tokio::test]
    async fn should_initialize_device_with_proper_commands() {
        // let mut out_buffer: [u8; 4] = [0; 4];
        // let mut len = Command::EnableDisplay(false).encode(&mut out_buffer);
        // let enable_display_command = vec![out_buffer[0..len]];
        let expections = build_expected_write_transaction(
            vec![
                encode_command(Command::EnableDisplay(false)),
                encode_command(Command::SetClockDivider(0x80)),
                encode_command(Command::SetMultiplexRatio(0x3F)),
            ],
            false,
        );
    }

    fn get_ssd1306_device(expectations: &Vec<I2cTransaction>) -> Ssd1306<I2cMock, 128, 64, 1024> {
        let i2c_device = I2cMock::new(expectations);
        Ssd1306::<_, 128, 64, 1024>::new(i2c_device, DEVICE_ADDRESS)
    }

    fn encode_command(command: Command) -> Vec<u8> {
        let mut buffer = [0u8; 4];
        let len = command.encode(&mut buffer);
        buffer[..len].to_vec()
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
        } else {
            transaction_data
                .iter()
                .map(|t| vec![I2cTransaction::write(DEVICE_ADDRESS, t.to_vec())])
                .flat_map(|f| f)
                .collect()
        }
    }
}
