use crate::display_driver::OutputPin;
use crate::frame::Frame;
use embassy_nrf::gpio::Output;
use embassy_time::Timer;

impl OutputPin for Output<'_> {
    fn set_high(&mut self) {
        Output::set_high(self);
    }
    fn set_low(&mut self) {
        Output::set_low(self);
    }
}

pub struct MicroBitLedDriver<P: OutputPin> {
    rows: [P; 5],
    cols: [P; 5],
}

impl<P: OutputPin> MicroBitLedDriver<P> {
    pub fn new(rows: [P; 5], cols: [P; 5]) -> Self {
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
