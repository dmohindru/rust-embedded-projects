#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals,
    spim::{self, Spim},
};
use embassy_time::{Delay, Timer};
use embedded_core::display_driver::{St7735, St7735Rgb565FrameBuffer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    TWISPI0 =>
        spim::InterruptHandler<peripherals::TWISPI0>;
});

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

    // let mut st7735_device = St7735::new(spi, dc, rst, frame_buffer, Delay);
}
