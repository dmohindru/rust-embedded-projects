use crate::display_driver::{
    st7735_driver::{command::Command, ColorFormat, Error},
    Encode,
};
use core::str::FromStr;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, spi::SpiDevice};
use heapless::String;

const SPI_WRITE_CHUNK_SIZE: usize = 512;

pub struct St7735<SPI, DC, RST, D, const WIDTH: usize, const HEIGHT: usize>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    D: DelayNs,
{
    device: SPI,
    dc: DC,
    rst: RST,
    delay: D,
    color_format: ColorFormat,
}

impl<SPI, DC, RST, D, const WIDTH: usize, const HEIGHT: usize>
    St7735<SPI, DC, RST, D, WIDTH, HEIGHT>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    D: DelayNs,
{
    pub fn new(device: SPI, dc: DC, rst: RST, delay: D) -> Self {
        Self {
            device,
            dc,
            rst,
            delay,
            color_format: ColorFormat::SixteenBPP,
        }
    }

    pub fn color_format(mut self, color_format: ColorFormat) -> Self {
        self.color_format = color_format;
        self
    }

    pub async fn initialize(&mut self) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.rst.set_high().map_err(|e| Error::Reset(e))?;
        self.write(Command::SWRESET).await?;
        self.write(Command::SLPOUT).await?;
        self.write(Command::FRMCTR1([0x01, 0x2C, 0x2D])).await?;
        self.write(Command::FRMCTR2([0x01, 0x2C, 0x2D])).await?;
        self.write(Command::FRMCTR3([0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D]))
            .await?;
        self.write(Command::INVCTR(0x07)).await?;
        self.write(Command::PWCTR1([0xA2, 0x02, 0x84])).await?;
        self.write(Command::PWCTR2(0xC5)).await?;
        self.write(Command::PWCTR3([0x0A, 0x00])).await?;
        self.write(Command::PWCTR4([0x8A, 0x2A])).await?;
        self.write(Command::PWCTR5([0x8A, 0xEE])).await?;
        self.write(Command::VMCTR1(0x0E)).await?;
        self.write(Command::MADCTL(0x00)).await?;
        self.write(Command::COLMOD(self.color_format.clone()))
            .await?;
        self.write(Command::GAMCTRP1([
            0x12, 0x1C, 0x10, 0x18, 0x33, 0x2C, 0x25, 0x28, 0x28, 0x27, 0x2F, 0x3C, 0x00, 0x03,
            0x03, 0x10,
        ]))
        .await?;
        self.write(Command::GAMCTRN1([
            0x12, 0x1C, 0x10, 0x18, 0x2D, 0x28, 0x23, 0x28, 0x28, 0x26, 0x2F, 0x3B, 0x00, 0x03,
            0x03, 0x10,
        ]))
        .await?;
        self.write(Command::DISPON).await?;
        self.write(Command::NORON).await?;

        Ok(())
    }

    pub async fn reset(&mut self) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.rst.set_low().map_err(|e| Error::Reset(e))?;
        self.delay.delay_ms(100).await;
        self.rst.set_high().map_err(|e| Error::Reset(e))?;
        Ok(())
    }

    /// Validation	Requirement
    /// Width	Greater than zero
    /// Height	Greater than zero
    /// Coordinates	Rectangle must fit within display dimensions
    /// Pixel data	Exactly width × height × 2 bytes for tightly packed data
    /// Stride	At least width × 2 bytes
    /// Strided data	At least (height - 1) × stride + width × 2 bytes
    /// Stride = Width X BytesPerPixel

    pub async fn update_rect(
        &mut self,
        x: u16,
        y: u16,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        Self::validate_coordinates(x, y)?;
        Self::validate_dimensions(width, height)?;
        // TODO other color format 18 BPP and 12 BPP to be handled later
        if self.color_format == ColorFormat::EighteenBPP
            || self.color_format == ColorFormat::TwelveBPP
        {
            todo!()
        }
        let sixteen_bpp_bytes = 2;
        let stride = width * sixteen_bpp_bytes;

        if data.len() < height * stride {
            return Err(Error::DataBuffer(Self::get_error_message(
                "Invalid data buffer size",
            )));
        }

        let x_end = x.saturating_add(width as u16 - 1).min((WIDTH - 1) as u16);
        // let y_end = y + height as u16 - 1;
        let y_end = y.saturating_add(height as u16 - 1).min((HEIGHT - 1) as u16);
        self.write(Command::CASET([x, x_end])).await?;
        self.write(Command::RASET([y, y_end])).await?;
        self.write(Command::RAMWR).await?;
        self.write_data(data).await?;
        Ok(())
    }

    pub async fn write(
        &mut self,
        command: Command,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        let mut buffer = [0u8; 17];
        let pos: usize = command.encode(&mut buffer);
        if pos > 1 {
            self.write_command(buffer[0]).await?;
            self.write_data(&buffer[1..pos]).await?;
        } else {
            self.write_command(buffer[0]).await?;
        }
        Ok(())
    }

    async fn write_command(
        &mut self,
        command: u8,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.dc.set_low().map_err(|e| Error::Dc(e))?;
        self.device
            .write(&[command])
            .await
            .map_err(|e| Error::Spi(e))?;

        Ok(())
    }

    async fn write_data(
        &mut self,
        args: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        self.dc.set_high().map_err(|e| Error::Dc(e))?;
        // self.device.write(args).await.map_err(|e| Error::Spi(e))?;

        for chunk in args.chunks(SPI_WRITE_CHUNK_SIZE) {
            self.device.write(chunk).await.map_err(|e| Error::Spi(e))?;
        }

        Ok(())
    }

    fn validate_coordinates(
        x: u16,
        y: u16,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        if x >= WIDTH as u16 {
            let error_msg = Self::get_error_message("Invalid x coordinates");
            return Err(Error::Coordinates(error_msg));
        }

        if y >= HEIGHT as u16 {
            let error_msg = Self::get_error_message("Invalid y coordinates");
            return Err(Error::Coordinates(error_msg));
        }

        Ok(())
    }

    fn validate_dimensions(
        width: usize,
        height: usize,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        if width > WIDTH {
            let error_msg = Self::get_error_message("Invalid width dimensions");
            return Err(Error::Coordinates(error_msg));
        }

        if height > HEIGHT {
            let error_msg = Self::get_error_message("Invalid height dimensions");
            return Err(Error::Coordinates(error_msg));
        }

        Ok(())
    }

    fn get_error_message(message: &str) -> String<64> {
        match String::<64>::from_str(message) {
            Ok(err_msg) => err_msg,
            Err(_) => String::<64>::from_str("Some error occurred").unwrap(),
        }
    }
}

#[cfg(test)]
impl<SPI, DC, RST, D, const WIDTH: usize, const HEIGHT: usize>
    St7735<SPI, DC, RST, D, WIDTH, HEIGHT>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    D: DelayNs,
{
    pub fn free(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_driver::st7735_driver::command::Command;
    use crate::display_driver::Encode;
    use crate::mocks::FakeDelay;
    use embedded_hal_mock::eh1::{
        digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
        spi::{Mock as SpiMock, Transaction as SpiTransaction},
    };
    const WIDTH: usize = 128;
    const HEIGHT: usize = 160;

    type MockSt7735 = St7735<SpiMock<u8>, PinMock, PinMock, FakeDelay, WIDTH, HEIGHT>;

    #[test]
    pub fn should_create_st7735_driver_struct_with_default_values() {
        let rst_pin_transactions: [PinTransaction; 0] = [];
        let dc_pin_transactions: [PinTransaction; 0] = [];
        let spi_transactions: Vec<SpiTransaction<u8>> = Vec::new();
        let device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );
        assert_eq!(device.color_format, ColorFormat::SixteenBPP);
        free_st7735_device(device);
    }

    #[test]
    pub fn should_create_and_update_st7735_struct_with_desired_color_mode_value() {
        let rst_pin_transactions: [PinTransaction; 0] = [];
        let dc_pin_transactions: [PinTransaction; 0] = [];
        let spi_transactions: Vec<SpiTransaction<u8>> = Vec::new();
        let device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );
        let updated_device = device.color_format(ColorFormat::EighteenBPP);
        assert_eq!(updated_device.color_format, ColorFormat::EighteenBPP);
        free_st7735_device(updated_device);
    }

    #[tokio::test]
    async fn should_initialize_st7735_with_proper_command_and_pin_transactions() {
        let rst_pin_transactions = [PinTransaction::set(PinState::High)];
        let dc_pin_transactions = [
            // SWRESET
            PinTransaction::set(PinState::Low),
            // SLPOUT
            PinTransaction::set(PinState::Low),
            // FRMCTR1
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // FRMCTR2
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // FRMCTR3
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // INVCTR
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // PWCTR1
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // PWCTR2
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // PWCTR3
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // PWCTR4
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // PWCTR5
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // VMCTR1
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // MADCTL
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // COLMOD
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // GAMCTRP1
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // GAMCTRN1
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // DISPON
            PinTransaction::set(PinState::Low),
            // NORON
            PinTransaction::set(PinState::Low),
        ];

        let spi_transactions = expected_initialization_command_spi_transactions();
        let mut device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );

        device.initialize().await.unwrap();
        free_st7735_device(device);
    }

    #[tokio::test]
    async fn should_reset_display_with_proper_pin_transactions() {
        let rst_pin_transactions = [
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];
        let dc_pin_transactions: [PinTransaction; 0] = [];
        let spi_transactions: Vec<SpiTransaction<u8>> = Vec::new();

        let mut device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );
        device.reset().await.unwrap();
        free_st7735_device(device);
    }

    #[tokio::test]
    async fn should_return_error_when_coordinates_x_y_value_are_not_within_allowed_range() {
        let rst_pin_transactions: [PinTransaction; 0] = [];
        let dc_pin_transactions: [PinTransaction; 0] = [];
        let spi_transactions: Vec<SpiTransaction<u8>> = Vec::new();
        let mut device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );

        let x_coordinate_error = device
            .update_rect(WIDTH as u16, 0, 10, 10, &[])
            .await
            .unwrap_err();

        match x_coordinate_error {
            Error::Coordinates(message) => {
                let expected_error_msg: String<64> =
                    String::from_str("Invalid x coordinates").unwrap();
                assert_eq!(message, expected_error_msg);
            }
            _ => {
                panic!("Unknown error");
            }
        }

        let y_coordinate_error = device
            .update_rect(0, HEIGHT as u16, 10, 10, &[])
            .await
            .unwrap_err();

        match y_coordinate_error {
            Error::Coordinates(message) => {
                let expected_error_msg: String<64> =
                    String::from_str("Invalid y coordinates").unwrap();

                assert_eq!(message, expected_error_msg);
            }
            _ => {
                panic!("Unknown error");
            }
        }
        free_st7735_device(device);
    }

    #[tokio::test]
    async fn should_return_error_when_rectangle_update_op_width_is_not_within_allowed_range() {
        let rst_pin_transactions: [PinTransaction; 0] = [];
        let dc_pin_transactions: [PinTransaction; 0] = [];
        let spi_transactions: Vec<SpiTransaction<u8>> = Vec::new();
        let mut device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );

        let width_error = device
            .update_rect(0, 0, WIDTH + 1, 10, &[])
            .await
            .unwrap_err();

        match width_error {
            Error::Coordinates(message) => {
                let expected_error_msg: String<64> =
                    String::from_str("Invalid width dimensions").unwrap();
                assert_eq!(message, expected_error_msg);
            }
            _ => {
                panic!("Unknown error");
            }
        }
        free_st7735_device(device);
    }

    #[tokio::test]
    async fn should_return_error_when_rectangle_update_op_height_is_not_within_allowed_range() {
        let rst_pin_transactions: [PinTransaction; 0] = [];
        let dc_pin_transactions: [PinTransaction; 0] = [];
        let spi_transactions: Vec<SpiTransaction<u8>> = Vec::new();
        let mut device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );

        let width_error = device
            .update_rect(0, 0, WIDTH, HEIGHT + 1, &[])
            .await
            .unwrap_err();

        match width_error {
            Error::Coordinates(message) => {
                let expected_error_msg: String<64> =
                    String::from_str("Invalid height dimensions").unwrap();
                assert_eq!(message, expected_error_msg);
            }
            _ => {
                panic!("Unknown error");
            }
        }
        free_st7735_device(device);
    }

    #[tokio::test]
    async fn should_return_error_when_supplied_data_slice_is_of_not_right_size_during_update_rect_ops(
    ) {
        let rst_pin_transactions: [PinTransaction; 0] = [];
        let dc_pin_transactions: [PinTransaction; 0] = [];
        let spi_transactions: Vec<SpiTransaction<u8>> = Vec::new();
        let mut device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );

        let frame_data: [u8; 90] = [0xFF; 90];
        let frame_data_error = device
            .update_rect(0, 0, 10, 10, &frame_data)
            .await
            .unwrap_err();

        match frame_data_error {
            Error::DataBuffer(message) => {
                let expected_error_msg: String<64> =
                    String::from_str("Invalid data buffer size").unwrap();
                assert_eq!(message, expected_error_msg);
            }
            _ => {
                panic!("Unknown error");
            }
        }
        free_st7735_device(device);
    }

    #[tokio::test]
    async fn should_flush_frame_data_with_proper_commands_and_pin_transactions_for_update_rect_ops()
    {
        let width = 20;
        let height = 20;
        let x: u16 = 10;
        let y: u16 = 20;

        let frame_data: [u8; 800] = [0xFF; 800];
        let rst_pin_transactions: [PinTransaction; 0] = [];
        let dc_pin_transactions = [
            // CASET
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // RASET
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            // RAMWR
            PinTransaction::set(PinState::Low),
            // Frame buffer data
            PinTransaction::set(PinState::High),
        ];
        let mut spi_transactions = Vec::new();
        concat_spi_transactions(&mut spi_transactions, Command::CASET([10, 29]));
        concat_spi_transactions(&mut spi_transactions, Command::RASET([20, 39]));
        concat_spi_transactions(&mut spi_transactions, Command::RAMWR);
        for chunk in frame_data.chunks(SPI_WRITE_CHUNK_SIZE) {
            let frame_buffer_data_transactions = vec![
                SpiTransaction::transaction_start(),
                SpiTransaction::write_vec(chunk.to_vec()),
                SpiTransaction::transaction_end(),
            ];
            spi_transactions.extend(frame_buffer_data_transactions);
        }

        let mut device = get_st7735_device(
            &rst_pin_transactions,
            &dc_pin_transactions,
            &spi_transactions,
        );
        device
            .update_rect(x, y, width, height, &frame_data)
            .await
            .unwrap();

        free_st7735_device(device);
    }

    fn expected_initialization_command_spi_transactions() -> Vec<SpiTransaction<u8>> {
        let mut transactions = Vec::new();
        concat_spi_transactions(&mut transactions, Command::SWRESET);
        concat_spi_transactions(&mut transactions, Command::SLPOUT);
        concat_spi_transactions(&mut transactions, Command::FRMCTR1([0x01, 0x2C, 0x2D]));
        concat_spi_transactions(&mut transactions, Command::FRMCTR2([0x01, 0x2C, 0x2D]));
        concat_spi_transactions(
            &mut transactions,
            Command::FRMCTR3([0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D]),
        );
        concat_spi_transactions(&mut transactions, Command::INVCTR(0x07));
        concat_spi_transactions(&mut transactions, Command::PWCTR1([0xA2, 0x02, 0x84]));
        concat_spi_transactions(&mut transactions, Command::PWCTR2(0xC5));
        concat_spi_transactions(&mut transactions, Command::PWCTR3([0x0A, 0x00]));
        concat_spi_transactions(&mut transactions, Command::PWCTR4([0x8A, 0x2A]));
        concat_spi_transactions(&mut transactions, Command::PWCTR5([0x8A, 0xEE]));
        concat_spi_transactions(&mut transactions, Command::VMCTR1(0x0E));
        concat_spi_transactions(&mut transactions, Command::MADCTL(0x00));
        concat_spi_transactions(&mut transactions, Command::COLMOD(ColorFormat::SixteenBPP));
        concat_spi_transactions(
            &mut transactions,
            Command::GAMCTRP1([
                0x12, 0x1C, 0x10, 0x18, 0x33, 0x2C, 0x25, 0x28, 0x28, 0x27, 0x2F, 0x3C, 0x00, 0x03,
                0x03, 0x10,
            ]),
        );
        concat_spi_transactions(
            &mut transactions,
            Command::GAMCTRN1([
                0x12, 0x1C, 0x10, 0x18, 0x2D, 0x28, 0x23, 0x28, 0x28, 0x26, 0x2F, 0x3B, 0x00, 0x03,
                0x03, 0x10,
            ]),
        );
        concat_spi_transactions(&mut transactions, Command::DISPON);
        concat_spi_transactions(&mut transactions, Command::NORON);

        transactions
    }

    fn encode_command(command: Command) -> Vec<u8> {
        let mut buffer = [0u8; 17];
        let len = command.encode(&mut buffer);
        buffer[..len].to_vec()
    }

    fn concat_spi_transactions(transactions: &mut Vec<SpiTransaction<u8>>, command: Command) {
        let bytes = encode_command(command);
        if bytes.len() > 1 {
            let command_transaction = vec![
                SpiTransaction::transaction_start(),
                SpiTransaction::write_vec(vec![bytes[0]]),
                SpiTransaction::transaction_end(),
            ];
            transactions.extend(command_transaction);
            let command_args_transactions = vec![
                SpiTransaction::transaction_start(),
                SpiTransaction::write_vec(bytes[1..].to_vec()),
                SpiTransaction::transaction_end(),
            ];
            transactions.extend(command_args_transactions);
        } else {
            let command_transaction = vec![
                SpiTransaction::transaction_start(),
                SpiTransaction::write_vec(vec![bytes[0]]),
                SpiTransaction::transaction_end(),
            ];
            transactions.extend(command_transaction);
        }
    }

    fn get_st7735_device(
        rst_pin_transactions: &[PinTransaction],
        dc_pin_transactions: &[PinTransaction],
        spi_transactions: &Vec<SpiTransaction<u8>>,
    ) -> MockSt7735 {
        let rst_pin = PinMock::new(rst_pin_transactions);
        let dc_pin = PinMock::new(dc_pin_transactions);
        let spi_device = SpiMock::new(spi_transactions);
        let fake_delay = FakeDelay::new();
        MockSt7735::new(spi_device, dc_pin, rst_pin, fake_delay)
    }

    fn free_st7735_device(st7735: MockSt7735) {
        let mut display = st7735.free();
        display.device.done();
        display.rst.done();
        display.dc.done();
    }
}
