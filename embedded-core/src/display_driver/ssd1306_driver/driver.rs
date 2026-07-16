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

    pub async fn initialize(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub async fn flush(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub async fn write_commands(&mut self, commands: &[u8]) -> Result<(), D::Error> {
        todo!()
    }

    pub async fn write_data(&mut self, data: &[u8]) -> Result<(), D::Error> {
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
    use crate::display_driver::{
        ssd1306_driver::command::{
            AddressMode, Command, CommandMode, DisplayMode, DisplaySize::Display128x64, PowerMode,
            ScanDirection, SegmentRemap,
        },
        Encode,
    };
    use embedded_hal::i2c::ErrorKind;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    static DEVICE_ADDRESS: u8 = 0x3D;

    #[tokio::test]
    async fn should_initialize_device_with_proper_commands() {
        let expectations = build_expected_write_transaction(
            vec![
                encode_command(Command::ControlByte(CommandMode::Control)),
                encode_command(Command::EnableDisplay(false)),
                encode_command(Command::SetClockDivider(0x80)),
                encode_command(Command::SetMultiplexRatio(0x3F)),
                encode_command(Command::SetDisplayOffset(0x00)),
                encode_command(Command::SetDisplayStartLine(0x00)),
                encode_command(Command::SetChargePump(PowerMode::InternalChargePump)),
                encode_command(Command::SetMemoryAddressMode(AddressMode::Horizontal)),
                // TODO may need to change these commands
                encode_command(Command::SetSegmentRemap(SegmentRemap::Remapped)),
                encode_command(Command::SetScanDirection(ScanDirection::BottomToTop)),
                // TODO ends here
                encode_command(Command::SetComPinConfig(Display128x64)),
                encode_command(Command::SetContrast(0xCF)),
                encode_command(Command::SetPreCharge(0xF1)),
                encode_command(Command::SetVComLevel),
                encode_command(Command::EnableRamContent(true)),
                encode_command(Command::SetDisplayMode(DisplayMode::Normal)),
                encode_command(Command::EnableDisplay(true)),
            ],
            false,
        );

        let mut ssd1306_driver = get_ssd1306_device(&expectations);
        ssd1306_driver.initialize().await.unwrap();
        ssd1306_driver.free().done();
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
