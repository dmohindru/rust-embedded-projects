use embassy_nrf::gpio::Output;
use embassy_time::Timer;

use crate::frame::Frame;

pub struct MicroBitLedDriver {
    rows: [Output<'static>; 5],
    cols: [Output<'static>; 5],
}

impl MicroBitLedDriver {
    pub fn new(rows: [Output<'static>; 5], cols: [Output<'static>; 5]) -> Self {
        MicroBitLedDriver { rows, cols }
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

            Timer::after_micros(300).await;
        }
    }
}
