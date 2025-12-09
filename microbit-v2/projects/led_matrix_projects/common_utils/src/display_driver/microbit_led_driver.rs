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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_driver::OutputPin as OutputPinTrait;
    use crate::frame::Frame;
    use async_trait::async_trait;
    use core::cell::Cell;
    use embedded_hal::digital::{InputPin, OutputPin};
    use embedded_hal_mock::eh1::pin::{
        Mock as PinMock, State as PinState, Transaction as PinTransaction,
    };
    use tokio::time::{sleep, Duration};

    pub struct TestDelay {
        // optionally record calls if you want to assert on them
        pub calls: Cell<u32>,
    }

    impl TestDelay {
        pub fn new() -> Self {
            Self {
                calls: Cell::new(0),
            }
        }
    }

    #[async_trait]
    impl AsyncDelay for TestDelay {
        async fn delay_micros(&mut self, us: u64) {
            self.calls.set(self.calls.get() + 1);
            sleep(Duration::from_micros(us)).await;
        }
    }

    struct TestPin {
        pin: PinMock,
    }

    impl TestPin {
        pub fn new(transactions: &[PinTransaction]) -> Self {
            Self {
                pin: PinMock::new(transactions),
            }
        }

        pub fn done(&mut self) {
            self.pin.done();
        }
    }

    impl OutputPinTrait for TestPin {
        fn set_high(&mut self) {
            self.pin.set_high().unwrap();
        }

        fn set_low(&mut self) {
            self.pin.set_low().unwrap();
        }
    }

    #[tokio::test]
    async fn test_led_matrix_driver() {
        let col1 = TestPin::new(&[]);
        let col2 = TestPin::new(&[]);
        let col3 = TestPin::new(&[]);
        let col4 = TestPin::new(&[]);
        let col5 = TestPin::new(&[]);
        let cols = [col1, col2, col3, col4, col5];

        let row1 = TestPin::new(&[]);
        let row2 = TestPin::new(&[]);
        let row3 = TestPin::new(&[]);
        let row4 = TestPin::new(&[]);
        let row5 = TestPin::new(&[]);
        let rows = [row1, row2, row3, row4, row5];

        let delay = TestDelay::new();

        let mut led_matrix_driver = MicroBitLedDriver::<TestPin, TestDelay>::new(rows, cols, delay);
        let frame = Frame::<5, 5>::new([0x01, 0x02, 0x03, 0x04, 0x05]);
        led_matrix_driver.render(&frame).await;

        for r in led_matrix_driver.rows.iter_mut() {
            r.done();
        }

        for c in led_matrix_driver.cols.iter_mut() {
            c.done();
        }
    }

    // #[test]
    // fn test() {
    //     // let err = MockError::Io(ErrorKind::NotConnected);
    //     let expectations = [
    //         PinTransaction::get(PinState::High),
    //         PinTransaction::get(PinState::Low),
    //         PinTransaction::get(PinState::High),
    //         PinTransaction::set(PinState::High),
    //         PinTransaction::set(PinState::Low),
    //         PinTransaction::set(PinState::High),
    //     ];

    //     let mut pin = PinMock::new(&expectations);
    //     // assert_eq!(pin.is_high().unwrap(), true);
    //     // assert_eq!(pin.is_low().unwrap(), true);
    //     // assert_eq!(pin.is_high().unwrap(), true);
    //     pin.is_high().unwrap();
    //     pin.is_low().unwrap();
    //     pin.is_high().unwrap();
    //     pin.set_high().unwrap();
    //     pin.set_low().unwrap();
    //     pin.set_high().unwrap();

    //     pin.done();
    // }
}
