#![no_std]
#![no_main]

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals,
    spim::{self, Spim},
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use embassy_time::Delay;
use embedded_core::display_driver::{ColorFormat, St7735, St7735Rgb565FrameBuffer};
use embedded_graphics::pixelcolor::Rgb565;

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
const WIDTH: usize = 128;
const HEIGHT: usize = 160;
const BYTES: usize = WIDTH * HEIGHT * 2;

type St7735SpiDevice = SpiDevice<'static, NoopRawMutex, MySpi, NoopOutputPin>;
type St7735Driver = St7735<
    St7735SpiDevice,
    Output<'static>,
    Output<'static>,
    St7735Rgb565FrameBuffer<WIDTH, HEIGHT, BYTES>,
    Delay,
    WIDTH,
    HEIGHT,
    BYTES,
>;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Configure SPI.
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M1;
    config.mode = embedded_hal::spi::MODE_0;

    // LCD_MOSI: P15 - P0.13
    // LCD_SCLK: P13 - P0.17
    // LCD_DC: P8 - P0.10
    // LCDRST: P16 - P1.02

    let spi = Spim::new_txonly(p.TWISPI0, Irqs, p.P0_17, p.P0_13, config);

    let frame_buffer = St7735Rgb565FrameBuffer::<WIDTH, HEIGHT, BYTES>::new();
    let rst = Output::new(p.P1_02, Level::High, OutputDrive::Standard);
    let dc = Output::new(p.P0_10, Level::Low, OutputDrive::Standard);

    // Put the raw SPI bus behind the async mutex.
    let spi_bus = SPI_BUS.init(Mutex::new(spi));

    // Wrap the bus as an embedded-hal-async SpiDevice.
    let spi_device = SpiDevice::new(spi_bus, NoopOutputPin);

    let mut st7735_driver: St7735Driver = St7735::new(spi_device, dc, rst, frame_buffer, Delay);
    info!("About to send Initialization commands");

    st7735_driver
        .initialize(ColorFormat::SixteenBPP)
        .await
        .unwrap();
    info!("Initialization command sent");

    // Black Background
    st7735_driver.fill_screen(Rgb565::new(0, 0, 0));

    // Red Top Left Square
    st7735_driver.draw_test_rectangles(0, 0, 20, 20, Rgb565::new(255, 0, 0));

    // Blue Top Right Square
    st7735_driver.draw_test_rectangles(108, 0, 20, 20, Rgb565::new(0, 0, 255));

    // Green Bottom Left Square
    st7735_driver.draw_test_rectangles(0, 140, 20, 20, Rgb565::new(0, 255, 0));

    // Yellow Bottom Right Square
    st7735_driver.draw_test_rectangles(108, 140, 20, 20, Rgb565::new(255, 255, 0));

    // Center white
    st7735_driver.draw_test_rectangles(54, 70, 20, 20, Rgb565::new(255, 255, 255));

    // Flush frame data to display
    st7735_driver.flush().await.unwrap();
}
