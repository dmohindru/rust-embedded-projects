use crate::display_driver::frame_buffer::{BinaryPixelFormat, FrameBuffer, Ssd1306PixelLayout};
use crate::display_driver::{
    ssd1306_driver::command::{
        AddressMode, Command, CommandMode, DisplayMode, DisplaySize, PowerMode, ScanDirection,
        SegmentRemap,
    },
    Encode,
};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_hal_async::i2c::I2c;

pub type Ssd1306FrameBuffer<const WIDTH: usize, const HEIGHT: usize, const BYTES: usize> =
    FrameBuffer<WIDTH, HEIGHT, BYTES, Ssd1306PixelLayout, BinaryPixelFormat>;

pub type Ssd1306_128x64<D> = Ssd1306<D, 128, 64, 1024, 1025>;

pub struct Ssd1306<
    D,
    const WIDTH: usize,
    const HEIGHT: usize,
    const BYTES: usize,
    const TX_BYTES: usize,
> where
    D: I2c,
{
    device: D,
    address: u8,
    // TODO Generalize over various screen sizes
    frame_buffer: Ssd1306FrameBuffer<WIDTH, HEIGHT, BYTES>,
}

impl<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize, const TX_BYTES: usize>
    Ssd1306<D, WIDTH, HEIGHT, BYTES, TX_BYTES>
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
        let mut buffer = [0u8; 32];
        let mut pos: usize = 0;
        pos += Command::ControlByte(CommandMode::Control).encode(&mut buffer[pos..]);
        pos += Command::EnableDisplay(false).encode(&mut buffer[pos..]);
        pos += Command::SetClockDivider(0x80).encode(&mut buffer[pos..]);
        pos += Command::SetMultiplexRatio(0x3F).encode(&mut buffer[pos..]);
        pos += Command::SetDisplayOffset(0x00).encode(&mut buffer[pos..]);
        pos += Command::SetDisplayStartLine(0x00).encode(&mut buffer[pos..]);
        pos += Command::SetChargePump(PowerMode::InternalChargePump).encode(&mut buffer[pos..]);
        pos += Command::SetMemoryAddressMode(AddressMode::Horizontal).encode(&mut buffer[pos..]);
        pos += Command::SetSegmentRemap(SegmentRemap::Remapped).encode(&mut buffer[pos..]);
        pos += Command::SetScanDirection(ScanDirection::BottomToTop).encode(&mut buffer[pos..]);
        pos += Command::SetComPinConfig(DisplaySize::Display128x64).encode(&mut buffer[pos..]);
        pos += Command::SetContrast(0xCF).encode(&mut buffer[pos..]);
        pos += Command::SetPreCharge(0xF1).encode(&mut buffer[pos..]);
        pos += Command::SetVComLevel.encode(&mut buffer[pos..]);
        pos += Command::EnableRamContent(true).encode(&mut buffer[pos..]);
        pos += Command::SetDisplayMode(DisplayMode::Normal).encode(&mut buffer[pos..]);
        pos += Command::EnableDisplay(true).encode(&mut buffer[pos..]);
        self.device.write(self.address, &buffer[..pos]).await?;
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<(), D::Error> {
        let mut buffer = [0u8; 10];
        let mut pos: usize = 0;
        pos += Command::ControlByte(CommandMode::Control).encode(&mut buffer[pos..]);
        pos += Command::SetColumnAddress([0x00, 0x7F]).encode(&mut buffer[pos..]);
        pos += Command::SetPageAddress([0x00, 0x07]).encode(&mut buffer[pos..]);
        self.device.write(self.address, &buffer[..pos]).await?;

        let mut tx_buffer: [u8; TX_BYTES] = [0; TX_BYTES];
        Command::ControlByte(CommandMode::Data).encode(&mut tx_buffer);
        tx_buffer[1..].copy_from_slice(self.frame_buffer.frame_data());
        self.device.write(self.address, &tx_buffer).await?;

        Ok(())
    }

    pub async fn invert_display(&mut self) -> Result<(), D::Error> {
        todo!()
    }

    pub async fn set_contrast(&mut self, contrast_value: u8) -> Result<(), D::Error> {
        todo!()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.frame_buffer.set_pixel(x, y, BinaryColor::On);
    }

    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        self.frame_buffer.set_pixel(x, y, BinaryColor::Off);
    }
}

#[cfg(test)]
impl<D, const WIDTH: usize, const HEIGHT: usize, const BYTES: usize, const TX_BYTES: usize>
    Ssd1306<D, WIDTH, HEIGHT, BYTES, TX_BYTES>
where
    D: I2c,
{
    pub fn free(self) -> (D, Ssd1306FrameBuffer<WIDTH, HEIGHT, BYTES>) {
        (self.device, self.frame_buffer)
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
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    static DEVICE_ADDRESS: u8 = 0x3D;

    #[tokio::test]
    async fn should_initialize_device_with_proper_commands() {
        let expectations = vec![I2cTransaction::write(
            DEVICE_ADDRESS,
            expected_initialization_command_bytes(),
        )];

        let mut ssd1306_driver = get_ssd1306_device(&expectations);
        ssd1306_driver.initialize().await.unwrap();
        ssd1306_driver.free().0.done();
    }

    fn get_ssd1306_device(expectations: &Vec<I2cTransaction>) -> Ssd1306_128x64<I2cMock> {
        let i2c_device = I2cMock::new(expectations);
        Ssd1306_128x64::<_>::new(i2c_device, DEVICE_ADDRESS)
    }

    fn encode_command(command: Command) -> Vec<u8> {
        let mut buffer = [0u8; 4];
        let len = command.encode(&mut buffer);
        buffer[..len].to_vec()
    }

    fn expected_initialization_command_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(encode_command(Command::ControlByte(CommandMode::Control)));
        bytes.extend(encode_command(Command::EnableDisplay(false)));
        bytes.extend(encode_command(Command::SetClockDivider(0x80)));
        bytes.extend(encode_command(Command::SetMultiplexRatio(0x3F)));
        bytes.extend(encode_command(Command::SetDisplayOffset(0x00)));
        bytes.extend(encode_command(Command::SetDisplayStartLine(0x00)));
        bytes.extend(encode_command(Command::SetChargePump(
            PowerMode::InternalChargePump,
        )));
        bytes.extend(encode_command(Command::SetMemoryAddressMode(
            AddressMode::Horizontal,
        )));
        bytes.extend(encode_command(Command::SetSegmentRemap(
            SegmentRemap::Remapped,
        )));
        bytes.extend(encode_command(Command::SetScanDirection(
            ScanDirection::BottomToTop,
        )));
        bytes.extend(encode_command(Command::SetComPinConfig(Display128x64)));
        bytes.extend(encode_command(Command::SetContrast(0xCF)));
        bytes.extend(encode_command(Command::SetPreCharge(0xF1)));
        bytes.extend(encode_command(Command::SetVComLevel));
        bytes.extend(encode_command(Command::EnableRamContent(true)));
        bytes.extend(encode_command(Command::SetDisplayMode(DisplayMode::Normal)));
        bytes.extend(encode_command(Command::EnableDisplay(true)));

        bytes
    }
}
