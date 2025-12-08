use crate::display_driver::{AsyncDelay, OutputPin};
use crate::frame::Frame;
use alloc::boxed::Box;
use async_trait::async_trait;
use embassy_nrf::gpio::Output;
use embassy_time::Timer;

pub struct EmbassyDelay;

#[async_trait]
impl AsyncDelay for EmbassyDelay {
    async fn delay_micros(&mut self, us: u64) {
        Timer::after_micros(us).await;
    }
}

impl OutputPin for Output<'_> {
    fn set_high(&mut self) {
        Output::set_high(self);
    }
    fn set_low(&mut self) {
        Output::set_low(self);
    }
}

pub struct MicroBitLedDriver<P: OutputPin, D: AsyncDelay> {
    rows: [P; 5],
    cols: [P; 5],
    delay: D,
}

impl<P: OutputPin, D: AsyncDelay> MicroBitLedDriver<P, D> {
    pub fn new(rows: [P; 5], cols: [P; 5], delay: D) -> Self {
        MicroBitLedDriver { rows, cols, delay }
    }

    pub async fn render(&mut self, frame: &Frame<5, 5>) {
        for row in 0..5 {
            for r in &mut self.rows {
                r.set_high();
            }

            self.rows[row].set_low();
            let row_bits = frame.get_row(row);

            for col in 0..5 {
                if (row_bits & (1 << (4 - col))) != 0 {
                    self.cols[col].set_low();
                } else {
                    self.cols[col].set_high();
                }
            }

            self.delay.delay_micros(300).await;
        }
    }
}
