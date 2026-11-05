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
use embassy_sync::{
    blocking_mutex::raw::{NoopRawMutex, ThreadModeRawMutex},
    channel::{Channel, Receiver, Sender},
    mutex::Mutex,
};
use embassy_time::{Delay, Timer};
use embedded_core::{
    cursor::StepCursorCircular,
    display_driver::st7735_driver::v2::St7735,
    frame::Direction,
    input_device::make_code_input::MakeCodeInput,
    output::NoopOutputPin,
    shift_register::{Hc165, SerialInput},
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    TWISPI0 =>
        spim::InterruptHandler<peripherals::TWISPI0>;
});

type MySpi = Spim<'static, peripherals::TWISPI0>;
type SpiBus = Mutex<NoopRawMutex, MySpi>;
type St7735SpiDevice = SpiDevice<'static, NoopRawMutex, MySpi, NoopOutputPin>;
type St7735Driver = St7735<St7735SpiDevice, Output<'static>, Output<'static>, Delay, WIDTH, HEIGHT>;

static SPI_BUS: StaticCell<SpiBus> = StaticCell::new();
static CHANNEL: Channel<ThreadModeRawMutex, Direction, 1> = Channel::new();
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

#[embassy_executor::task]
async fn input_scanner(
    mut make_code_input: MakeCodeInput<Hc165<Input<'static>, Output<'static>, Delay>>,
    sender: Sender<'static, ThreadModeRawMutex, Direction, 1>,
) {
    loop {
        let report = make_code_input.read_input_report().await.unwrap();
        if report.a_btn_pressed {
            sender.send(Direction::Left).await;
        }
        if report.b_btn_pressed {
            sender.send(Direction::Right).await;
        }
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::task]
async fn frame_updater(
    mut st7735_driver: St7735Driver,
    receiver: Receiver<'static, ThreadModeRawMutex, Direction, 1>,
) {
    let mut circular_cursor = StepCursorCircular::new(3, 0, 1, 4);
    let coordinates: [(u16, u16, &[u8; 800]); 4] = [
        // TOP LEFT - Red Color
        (0, 0, &RED_FRAME_BUFFER),
        // TOP RIGHT - Green Color
        (108, 0, &GREEN_FRAME_BUFFER),
        // BOTTOM LEFT - Blue Color
        (0, 140, &BLUE_FRAME_BUFFER),
        // BOTTOM RIGHT - Yellow Color
        (108, 140, &YELLOW_FRAME_BUFFER),
    ];
    loop {
        let button_pressed = receiver.receive().await;
        let previous_index = circular_cursor.current_value() as usize;
        circular_cursor.move_step(button_pressed);
        let new_index = circular_cursor.current_value() as usize;
        let current_coordinates = coordinates[new_index];
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // ***Configure SPI and ST7735 display.***
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

    // ***Configure Input device ***
    let shift_load = Output::new(p.P0_09, Level::High, OutputDrive::Standard);
    let clk = Output::new(p.P1_00, Level::High, OutputDrive::Standard);
    let data_in = Input::new(p.P0_01, Pull::Down);
    let hc165_device = Hc165::new(shift_load, clk, data_in, Delay, 4).unwrap();
    let make_code_input = MakeCodeInput::new(hc165_device);

    // *** Sender & Receiver ***
    let input_btn_sender = CHANNEL.sender();
    let frame_updater_receiver = CHANNEL.receiver();

    st7735_driver.initialize().await.unwrap();
    info!("Initialization command sent");
    st7735_driver
        .update_rect(0, 0, WIDTH, HEIGHT, &SCREEN_BACKGROUND_BUFFER)
        .await
        .unwrap();

    st7735_driver
        .update_rect(0, 0, RECT_WIDTH, RECT_HEIGHT, &RED_FRAME_BUFFER)
        .await
        .unwrap();

    spawner
        .spawn(input_scanner(make_code_input, input_btn_sender))
        .expect("Failed to spawn input scanner task");

    spawner
        .spawn(frame_updater(st7735_driver, frame_updater_receiver))
        .expect("Failed to spawn frame updater task");

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
