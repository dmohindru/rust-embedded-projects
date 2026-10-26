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
use embassy_time::{Delay, Timer};
use embedded_core::display_driver::{St7735, St7735Rgb565FrameBuffer};
use embedded_core::output::NoopOutputPin;
use embedded_hal::digital::v2::OutputPin;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    TWISPI0 =>
        spim::InterruptHandler<peripherals::TWISPI0>;
});

// This is valid ONLY if the hardware holds display CS low.
struct StaticCs;
impl OutputPin for StaticCs {
    type Error = core::convert::Infallible;
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

type MySpi = Spim<'static, peripherals::TWISPI0>;
type SpiBus = Mutex<NoopRawMutex, MySpi>;
static SPI_BUS: StaticCell<SpiBus> = StaticCell::new();

type St7735SpiDevice = SpiDevice<'static, NoopRawMutex, MySpi, NoopOutputPin>;
type St7735Driver = St7735<
    St7735SpiDevice,
    Output<'static>,
    Output<'static>,
    St7735Rgb565FrameBuffer<162, 132, 42768>,
    Delay,
    162,
    132,
    42768,
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

    let mut spi = Spim::new_txonly(p.TWISPI0, Irqs, p.P0_17, p.P0_13, config);
    // Width: 162
    // Height: 132
    // Bytes = 162 X 132 X 2 = 42768
    let frame_buffer = St7735Rgb565FrameBuffer::<162, 132, 42768>::new();
    let mut rst = Output::new(p.P1_02, Level::High, OutputDrive::Standard);
    let mut dc = Output::new(p.P0_10, Level::Low, OutputDrive::Standard);
    // let mut cs = Output::new(p.P0_11, Level::Low, OutputDrive::Standard);

    // Put the raw SPI bus behind the async mutex.
    let spi_bus = SPI_BUS.init(Mutex::new(spi));

    // Wrap the bus as an embedded-hal-async SpiDevice.
    let spi_device = SpiDevice::new(spi_bus, NoopOutputPin);

    let st7735_driver: St7735Driver = St7735::new(spi_device, dc, rst, frame_buffer, Delay);
}
