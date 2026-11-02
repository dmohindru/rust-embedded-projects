use crate::display_driver::st7735_driver::{ColorFormat, Error};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, spi::SpiDevice};

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
        todo!()
    }

    // Validation	Requirement
    // Width	Greater than zero
    // Height	Greater than zero
    // Coordinates	Rectangle must fit within display dimensions
    // Pixel data	Exactly width × height × 2 bytes for tightly packed data
    // Stride	At least width × 2 bytes
    // Strided data	At least (height - 1) × stride + width × 2 bytes
    // Stride = Width X BytesPerPixel

    pub async fn update_rect(
        &mut self,
        x: u16,
        y: u16,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    pub async fn update_rect_stride(
        &mut self,
        x: u16,
        y: u16,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        todo!()
    }

    async fn set_address_window(
        &mut self,
        x: u16,
        y: u16,
        width: usize,
        height: usize,
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        // Validate region and data length
        // Configure address window
        // Issue RAMWR
        // Transfer pixel data
        todo!()
    }

    async fn write_pixel_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), Error<SPI::Error, DC::Error, RST::Error>> {
        // Validate region, stride and data length
        // Configure address window
        // Issue RAMWR
        // Transfer each row, respecting stride
        todo!()
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
        todo!()
    }

    #[tokio::test]
    async fn should_return_error_when_coordinates_x_y_value_are_not_within_allowed_range() {
        todo!()
    }

    #[tokio::test]
    async fn should_return_error_when_rectangle_update_op_width_is_not_within_allowed_range() {
        todo!()
    }

    #[tokio::test]
    async fn should_return_error_when_rectangle_update_op_height_is_not_within_allowed_range() {
        todo!()
    }

    #[tokio::test]
    async fn should_return_error_when_supplied_data_slice_is_of_not_right_size_during_update_rect_ops(
    ) {
        todo!()
    }

    #[tokio::test]
    async fn should_return_error_when_supplied_data_slice_is_of_not_right_size_during_update_rect_stride_ops(
    ) {
        todo!()
    }

    #[tokio::test]
    async fn should_flush_frame_data_with_proper_commands_and_pin_transactions_for_update_rect_ops()
    {
        todo!()
    }

    #[tokio::test]
    async fn should_flush_frame_data_with_proper_commands_and_pin_transactions_for_update_rect_stride_ops(
    ) {
        todo!()
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
