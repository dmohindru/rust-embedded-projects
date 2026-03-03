use crate::frame::Frame;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;

pub struct LedMatrixDriver<P, D, const R: usize, const C: usize>
where
    P: OutputPin,
    D: DelayNs,
{
    rows: [P; R],
    cols: [P; C],
    delay: D,
}

impl<P, D, const R: usize, const C: usize> LedMatrixDriver<P, D, R, C>
where
    P: OutputPin,
    D: DelayNs,
{
    pub fn new(rows: [P; R], cols: [P; C], delay: D) -> Self {
        LedMatrixDriver { rows, cols, delay }
    }

    pub async fn render(&mut self, frame: &Frame<R, C>) {
        for row in 0..R {
            // Turn off all rows
            for r in &mut self.rows {
                r.set_low().unwrap();
            }

            // Activate current row
            self.rows[row].set_high().unwrap();
            let row_bits = frame.get_row(row);

            for col in 0..C {
                if (row_bits & (1 << (C - 1 - col))) != 0 {
                    self.cols[col].set_low().unwrap();
                } else {
                    self.cols[col].set_high().unwrap();
                }
            }

            self.delay.delay_ms(1000).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;
    use embedded_hal_mock::eh1::digital::Mock as PinMock;
    use embedded_hal_mock::eh1::digital::{State, Transaction};
    use tokio::time::{sleep, Duration};

    pub struct FakeDelay {
        pub calls: Cell<u32>,
    }

    impl FakeDelay {
        pub fn new() -> Self {
            Self {
                calls: Cell::new(0),
            }
        }
    }

    impl DelayNs for FakeDelay {
        async fn delay_ns(&mut self, us: u32) {
            self.calls.set(self.calls.get() + 1);
            sleep(Duration::from_micros(us as u64)).await;
        }

        async fn delay_ms(&mut self, ms: u32) {
            self.delay_ns(ms).await;
        }
    }

    #[tokio::test]
    async fn test_led_matrix_driver() {
        let col1 = PinMock::new(&[
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::High),
        ]);
        let col2 = PinMock::new(&[
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::High),
        ]);
        let col3 = PinMock::new(&[
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::High),
        ]);
        let col4 = PinMock::new(&[
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::High),
        ]);
        let col5 = PinMock::new(&[
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::High),
            Transaction::set(State::Low),
        ]);
        let cols = [col1, col2, col3, col4, col5];

        let row1 = PinMock::new(&[
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
        ]);
        let row2 = PinMock::new(&[
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
        ]);
        let row3 = PinMock::new(&[
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
        ]);
        let row4 = PinMock::new(&[
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::Low),
        ]);
        let row5 = PinMock::new(&[
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::Low),
            Transaction::set(State::High),
        ]);
        let rows = [row1, row2, row3, row4, row5];

        let delay = FakeDelay::new();

        let mut led_matrix_driver =
            LedMatrixDriver::<PinMock, FakeDelay, 5, 5>::new(rows, cols, delay);
        let frame = Frame::<5, 5>::new([0x10, 0x08, 0x04, 0x02, 0x01]);
        led_matrix_driver.render(&frame).await;

        for r in led_matrix_driver.rows.iter_mut() {
            r.done();
        }

        for c in led_matrix_driver.cols.iter_mut() {
            c.done();
        }

        assert_eq!(led_matrix_driver.delay.calls.get(), 5);
    }
}
