#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    peripherals,
    spim::{self, Spim},
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::Delay;
use embedded_core::{
    display_driver::st7735_driver::{v2::St7735, ColorFormat},
    input_device::make_code_input::MakeCodeInput,
    shift_register::Hc165,
};
use embedded_graphics::pixelcolor::{IntoStorage, Rgb565, RgbColor};

use embedded_core::output::NoopOutputPin;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    TWISPI0 =>
        spim::InterruptHandler<peripherals::TWISPI0>;
});

type MySpi = Spim<'static, peripherals::TWISPI0>;
type SpiBus = Mutex<NoopRawMutex, MySpi>;
static SPI_BUS: StaticCell<SpiBus> = StaticCell::new();

// Color codes
const RED_COLOR: u16 = 0xF800;
const GREEN_COLOR: u16 = 0x07E0;
const BLUE_COLOR: u16 = 0x001F;
const YELLOW_COLOR: u16 = 0xFFE0;
const BLACK_COLOR: u16 = 0x0000;

const WIDTH: usize = 128;
const HEIGHT: usize = 160;
const RECT_WIDTH: usize = 20;
const RECT_HEIGHT: usize = 20;
const BYTES_PER_PIXEL: usize = 2;
const RECT_FRAME_BUFFER_SIZE: usize = RECT_WIDTH * RECT_HEIGHT * BYTES_PER_PIXEL;
const SCREEN_FRAME_BUFFER_SIZE: usize = WIDTH * HEIGHT * BYTES_PER_PIXEL;

static RED_FRAME_BUFFER: [u8; RECT_FRAME_BUFFER_SIZE] = create_rect_frame(RED_COLOR);
static GREEN_FRAME_BUFFER: [u8; RECT_FRAME_BUFFER_SIZE] = create_rect_frame(GREEN_COLOR);
static BLUE_FRAME_BUFFER: [u8; RECT_FRAME_BUFFER_SIZE] = create_rect_frame(BLUE_COLOR);
static YELLOW_FRAME_BUFFER: [u8; RECT_FRAME_BUFFER_SIZE] = create_rect_frame(YELLOW_COLOR);
static BLACK_FRAME_BUFFER: [u8; RECT_FRAME_BUFFER_SIZE] = create_rect_frame(YELLOW_COLOR);

static SCREEN_BACKGROUND_BUFFER: [u8; SCREEN_FRAME_BUFFER_SIZE] = create_rect_frame(BLACK_COLOR);

type St7735SpiDevice = SpiDevice<'static, NoopRawMutex, MySpi, NoopOutputPin>;
type St7735Driver = St7735<St7735SpiDevice, Output<'static>, Output<'static>, Delay, WIDTH, HEIGHT>;

const fn create_rect_frame<const SIZE: usize>(color: u16) -> [u8; SIZE] {
    let mut buffer = [0u8; SIZE];
    let high_byte = (color >> 8) as u8;
    let low_byte = color as u8;
    let mut i = 0;
    while i < SIZE {
        buffer[i] = if i % 2 == 0 { high_byte } else { low_byte };

        i += 1;
    }
    buffer
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Configure SPI and ST7735 display.
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M1;
    config.mode = embedded_hal::spi::MODE_0;

    // LCD_MOSI: P15 - P0.13
    // LCD_SCLK: P13 - P0.17
    // LCD_DC: P8 - P0.10
    // LCDRST: P16 - P1.02

    let spi = Spim::new_txonly(p.TWISPI0, Irqs, p.P0_17, p.P0_13, config);

    let rst = Output::new(p.P1_02, Level::High, OutputDrive::Standard);
    let dc = Output::new(p.P0_10, Level::Low, OutputDrive::Standard);

    // Put the raw SPI bus behind the async mutex.
    let spi_bus = SPI_BUS.init(Mutex::new(spi));

    // Wrap the bus as an embedded-hal-async SpiDevice.
    let spi_device = SpiDevice::new(spi_bus, NoopOutputPin);

    let mut st7735_driver: St7735Driver = St7735::new(spi_device, dc, rst, Delay);
    info!("About to send Initialization commands");

    // Configure Input device
    let shift_load = Output::new(p.P0_09, Level::High, OutputDrive::Standard);
    let clk = Output::new(p.P1_00, Level::High, OutputDrive::Standard);
    let data_in = Input::new(p.P0_01, Pull::Down);
    let hc165_device = Hc165::new(shift_load, clk, data_in, Delay, 4).unwrap();
    let mut make_code_input = MakeCodeInput::new(hc165_device);

    st7735_driver.initialize().await.unwrap();
    info!("Initialization command sent");

    // Black Background
    // st7735_driver.fill_screen(Rgb565::new(0, 0, 0));

    // // Red Top Left Square
    // st7735_driver.draw_test_rectangles(0, 0, 20, 20, Rgb565::new(255, 0, 0));

    // // Blue Top Right Square
    // st7735_driver.draw_test_rectangles(108, 0, 20, 20, Rgb565::new(0, 0, 255));

    // // Green Bottom Left Square
    // st7735_driver.draw_test_rectangles(0, 140, 20, 20, Rgb565::new(0, 255, 0));

    // // Yellow Bottom Right Square
    // st7735_driver.draw_test_rectangles(108, 140, 20, 20, Rgb565::new(255, 255, 0));

    // // Center white
    // st7735_driver.draw_test_rectangles(54, 70, 20, 20, Rgb565::new(255, 255, 255));

    // // Flush frame data to display
    // st7735_driver.flush().await.unwrap();
}
