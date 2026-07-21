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
    inverted_display: bool,
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
            inverted_display: false,
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
        let mut buffer = [0u8; 10];
        let mut pos: usize = 0;
        if self.inverted_display {
            pos += Command::ControlByte(CommandMode::Control).encode(&mut buffer[pos..]);
            pos += Command::SetDisplayMode(DisplayMode::Normal).encode(&mut buffer[pos..]);
            self.device.write(self.address, &buffer[..pos]).await?;
        } else {
            pos += Command::ControlByte(CommandMode::Control).encode(&mut buffer[pos..]);
            pos += Command::SetDisplayMode(DisplayMode::Inverted).encode(&mut buffer[pos..]);
            self.device.write(self.address, &buffer[..pos]).await?;
        }
        self.inverted_display = !self.inverted_display;
        Ok(())
    }

    pub async fn set_contrast(&mut self, contrast_value: u8) -> Result<(), D::Error> {
        let mut buffer = [0u8; 10];
        let mut pos: usize = 0;
        pos += Command::ControlByte(CommandMode::Control).encode(&mut buffer[pos..]);
        pos += Command::SetContrast(contrast_value).encode(&mut buffer[pos..]);
        self.device.write(self.address, &buffer[..pos]).await?;
        Ok(())
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

    #[tokio::test]
    async fn should_flush_data_with_proper_commands() {
        let mut flush_command_bytes = Vec::new();
        flush_command_bytes.extend(encode_command(Command::ControlByte(CommandMode::Control)));
        flush_command_bytes.extend(encode_command(Command::SetColumnAddress([0x00, 0x7F])));
        flush_command_bytes.extend(encode_command(Command::SetPageAddress([0x00, 0x07])));
        let expected_commands = I2cTransaction::write(DEVICE_ADDRESS, flush_command_bytes);

        let mut flush_data_bytes = Vec::new();
        flush_data_bytes.extend(encode_command(Command::ControlByte(CommandMode::Data)));
        flush_data_bytes.extend(vec![0; 1024]);
        let expected_data_commands = I2cTransaction::write(DEVICE_ADDRESS, flush_data_bytes);

        let expectations = vec![expected_commands, expected_data_commands];

        let mut ssd1306_driver = get_ssd1306_device(&expectations);
        ssd1306_driver.flush().await.unwrap();
        ssd1306_driver.free().0.done();
    }

    #[tokio::test]
    async fn should_invert_display_with_proper_commands() {
        let mut invert_display_command_bytes = Vec::new();
        invert_display_command_bytes
            .extend(encode_command(Command::ControlByte(CommandMode::Control)));
        invert_display_command_bytes.extend(encode_command(Command::SetDisplayMode(
            DisplayMode::Inverted,
        )));
        let expected_invert_display_command =
            I2cTransaction::write(DEVICE_ADDRESS, invert_display_command_bytes);

        let mut normal_display_command_bytes = Vec::new();
        normal_display_command_bytes
            .extend(encode_command(Command::ControlByte(CommandMode::Control)));
        normal_display_command_bytes
            .extend(encode_command(Command::SetDisplayMode(DisplayMode::Normal)));

        let expected_normal_display_command =
            I2cTransaction::write(DEVICE_ADDRESS, normal_display_command_bytes);

        let expectations = vec![
            expected_invert_display_command,
            expected_normal_display_command,
        ];

        let mut ssd1306_driver = get_ssd1306_device(&expectations);
        ssd1306_driver.invert_display().await.unwrap();
        ssd1306_driver.invert_display().await.unwrap();
        ssd1306_driver.free().0.done();
    }

    #[tokio::test]
    async fn should_set_contrast_with_proper_commands() {
        let contrast_value: u8 = 0x08;
        let mut set_contrast_command_bytes = Vec::new();
        set_contrast_command_bytes
            .extend(encode_command(Command::ControlByte(CommandMode::Control)));
        set_contrast_command_bytes.extend(encode_command(Command::SetContrast(contrast_value)));
        let expected_set_contrast_command =
            I2cTransaction::write(DEVICE_ADDRESS, set_contrast_command_bytes);

        let expectations = vec![expected_set_contrast_command];

        let mut ssd1306_driver = get_ssd1306_device(&expectations);
        ssd1306_driver.set_contrast(contrast_value).await.unwrap();
        ssd1306_driver.free().0.done();
    }

    #[test]
    fn should_set_pixel_in_framebuffer() {
        let mut ssd1306_driver = get_ssd1306_device(&Vec::new());
        ssd1306_driver.set_pixel(0, 0);
        ssd1306_driver.set_pixel(0, 2);
        ssd1306_driver.set_pixel(0, 4);
        ssd1306_driver.set_pixel(0, 6);
        let (mut device, framebuffer) = ssd1306_driver.free();
        let byte = framebuffer.frame_data()[0];
        assert_eq!(0x55, byte);
        device.done();
    }

    #[test]
    fn should_clear_pixel_in_framebuffer() {
        let mut ssd1306_driver = get_ssd1306_device(&Vec::new());
        ssd1306_driver.set_pixel(0, 0);
        ssd1306_driver.set_pixel(0, 2);
        ssd1306_driver.set_pixel(0, 4);
        ssd1306_driver.set_pixel(0, 6);
        ssd1306_driver.clear_pixel(0, 0);
        ssd1306_driver.clear_pixel(0, 2);
        let (mut device, framebuffer) = ssd1306_driver.free();
        let byte = framebuffer.frame_data()[0];
        assert_eq!(0x50, byte);
        device.done();
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
