#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Output, Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    mutex::{Mutex, MutexGuard},
};
use embassy_time::Timer;
use embedded_alloc::Heap;
use embedded_core::cursor::StepCursorCircular;
use embedded_core::display_driver::LedMatrixDriver;
use embedded_core::frame::{Direction, Frame, FrameCursorCircular};
use embedded_core::input_device::button::DebouncedButton;
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

const LED_MATRIX_ROWS: usize = 5;
const LED_MATRIX_COLS: usize = 5;
const TWELVE: [u32; 5] = [0b00100, 0b00100, 0b00100, 0b00000, 0b00000];
const ONE: [u32; 5] = [0b00001, 0b00010, 0b00100, 0b00000, 0b00000];
const THREE: [u32; 5] = [0b00000, 0b00000, 0b00111, 0b00000, 0b00000];
const FIVE: [u32; 5] = [0b00000, 0b00000, 0b00100, 0b00010, 0b00001];
const SIX: [u32; 5] = [0b00000, 0b00000, 0b00100, 0b00100, 0b00100];
const SEVEN: [u32; 5] = [0b00000, 0b00000, 0b00100, 0b01000, 0b10000];
const NINE: [u32; 5] = [0b00000, 0b00000, 0b11100, 0b00000, 0b00000];
const ELEVEN: [u32; 5] = [0b10000, 0b01000, 0b00100, 0b00000, 0b00000];

type FrameCursorType = Mutex<ThreadModeRawMutex, Option<FrameCursorCircular<8, 5, 5>>>;
type StepCursorType = Mutex<ThreadModeRawMutex, Option<StepCursorCircular>>;
type DirectionType = Mutex<ThreadModeRawMutex, Direction>;

static FRAME_CURSOR: FrameCursorType = Mutex::new(None);
static STEP_CURSOR: StepCursorType = Mutex::new(None);
static DIRECTION: DirectionType = Mutex::new(Direction::Right);

//--------------------
// Led Refresh task
//--------------------
#[embassy_executor::task]
async fn led_refresh_task(
    mut driver: LedMatrixDriver<
        Output<'static>,
        embassy_time::Delay,
        LED_MATRIX_ROWS,
        LED_MATRIX_COLS,
    >,
) {
    loop {
        // Read frame once per scan → lock only once
        let frame = {
            let frame_opt = FRAME_CURSOR.lock().await;
            frame_opt.as_ref().unwrap().current_frame().clone()
        };

        driver.render(&frame).await;
    }
}

//--------------------
// Left Button Task
//--------------------
#[embassy_executor::task]
async fn left_button_task(mut button: DebouncedButton<Input<'static>, embassy_time::Delay>) {
    button
        .wait(|| async {
            {
                let mut direction: MutexGuard<'_, ThreadModeRawMutex, Direction> =
                    DIRECTION.lock().await;
                *direction = direction.toggle();
            }
        })
        .await;
}

//--------------------
// Right Button Task
//--------------------
#[embassy_executor::task]
async fn right_button_task(mut button: DebouncedButton<Input<'static>, embassy_time::Delay>) {
    button
        .wait(|| async {
            {
                let mut step_cursor_option: MutexGuard<
                    '_,
                    ThreadModeRawMutex,
                    Option<StepCursorCircular>,
                > = STEP_CURSOR.lock().await;
                if let Some(step_cursor) = step_cursor_option.as_mut() {
                    step_cursor.move_step(Direction::Right);
                }
            }
        })
        .await;
}

//--------------------
// Timer Task
//--------------------
#[embassy_executor::task]
async fn timer_task() {
    {
        loop {
            // ---- 1. Read delay from step cursor ----
            let delay_ms = {
                let step_cursor_guard: MutexGuard<
                    '_,
                    ThreadModeRawMutex,
                    Option<StepCursorCircular>,
                > = STEP_CURSOR.lock().await;
                step_cursor_guard
                    .as_ref()
                    .map(|cursor| cursor.current_value())
                    .unwrap_or(2000)
            }; // Step cursor dropped here

            // ---- 2. Sleep (NO LOCKS HELD) ----
            Timer::after_millis(delay_ms as u64).await;

            // ---- 3. Read direction (copy, cheap) ----
            let direction = {
                let direction_guard: MutexGuard<'_, ThreadModeRawMutex, Direction> =
                    DIRECTION.lock().await;
                *direction_guard
            }; // direction guard dropped here

            // ---- 4. Move frame cursor ----
            {
                let mut frame_cursor_guard: MutexGuard<
                    '_,
                    ThreadModeRawMutex,
                    Option<FrameCursorCircular<8, 5, 5>>,
                > = FRAME_CURSOR.lock().await;
                if let Some(frame_cursor) = frame_cursor_guard.as_mut() {
                    frame_cursor.move_index(direction);
                }
            } // frame_cursor_guard dropped here
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 8 KB heap — you choose the size
    const HEAP_SIZE: usize = 8 * 1024;
    static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    unsafe {
        ALLOCATOR.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
    }

    let p = embassy_nrf::init(Default::default());
    let btn_a = Input::new(p.P0_14, Pull::Up);
    let btn_b = Input::new(p.P0_23, Pull::Up);
    let debounced_button_a = DebouncedButton::new(btn_a, embassy_time::Delay, 20);
    let debounced_button_b = DebouncedButton::new(btn_b, embassy_time::Delay, 20);

    let f1: Frame<5, 5> = Frame::<5, 5>::new(TWELVE);
    let f2: Frame<5, 5> = Frame::<5, 5>::new(ONE);
    let f3: Frame<5, 5> = Frame::<5, 5>::new(THREE);
    let f4: Frame<5, 5> = Frame::<5, 5>::new(FIVE);
    let f5: Frame<5, 5> = Frame::<5, 5>::new(SIX);
    let f6: Frame<5, 5> = Frame::<5, 5>::new(SEVEN);
    let f7: Frame<5, 5> = Frame::<5, 5>::new(NINE);
    let f8: Frame<5, 5> = Frame::<5, 5>::new(ELEVEN);
    let frames = [f1, f2, f3, f4, f5, f6, f7, f8];
    let frame_cursor = FrameCursorCircular::<8, 5, 5>::new(&frames);
    {
        *(FRAME_CURSOR.lock().await) = Some(frame_cursor);
    }
    let step_cursor = StepCursorCircular::new(2000, 500, 250, 7);
    {
        *(STEP_CURSOR.lock().await) = Some(step_cursor);
    }

    // -----------------
    // LED matrix pins
    // -----------------
    let rows = [
        Output::new(
            p.P0_21,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_22,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_15,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_24,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_19,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
    ];

    let cols = [
        Output::new(
            p.P0_28,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_11,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_31,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P1_05,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_30,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
    ];

    let led_driver = LedMatrixDriver::new(rows, cols, embassy_time::Delay);

    spawner
        .spawn(left_button_task(debounced_button_a))
        .expect("Failed to spawn button A task");
    spawner
        .spawn(right_button_task(debounced_button_b))
        .expect("Failed to spawn button B task");
    spawner
        .spawn(timer_task())
        .expect("Failed to spawn timer task");

    spawner
        .spawn(led_refresh_task(led_driver))
        .expect("Failed to spawn led refresh task");
}
