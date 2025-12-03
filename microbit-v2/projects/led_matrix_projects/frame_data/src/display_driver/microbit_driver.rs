use crate::display_driver::{DisplayDriver, PinOutput};
use crate::frame::Frame;

#[cfg(test)]
use embedded_hal::digital::OutputPin;

#[cfg(not(test))]
use embassy_nrf::gpio::Output;

#[cfg(not(test))]
impl PinOutput for Output<'_> {
    fn set_high(&mut self) {
        Output::set_high(self)
    }

    fn set_low(&mut self) {
        Output::set_low(self)
    }
}

#[cfg(test)]
impl<P> PinOutput for P
where
    P: OutputPin,
{
    fn set_high(&mut self) {
        self.set_high().unwrap(); // unwrap is fine for tests
    }

    fn set_low(&mut self) {
        self.set_low().unwrap();
    }
}

#[cfg(not(test))]
pub struct EmbassyDelay;

#[cfg(not(test))]
#[async_trait]
impl Delay for EmbassyDelay {
    async fn delay_us(&mut self, us: u32) {
        embassy_time::Timer::after_micros(us as u64).await;
    }
}

#[cfg(test)]
pub struct NoopDelay;

#[cfg(test)]
#[async_trait]
impl Delay for NoopDelay {
    async fn delay_us(&mut self, _us: u32) {
        // nothing — makes tests run instantly
    }
}

pub struct MicrobitDriver<'a> {
    rows: [&'a mut dyn PinOutput; 5],
    cols: [&'a mut dyn PinOutput; 5],
    delay: &'a mut dyn Delay,
    row_index: usize,
}

impl<'a> MicrobitDriver<'a> {
    pub fn new(
        rows: [&'a mut dyn PinOutput; 5],
        cols: [&'a mut dyn PinOutput; 5],
        delay: &'a mut dyn Delay,
    ) -> Self {
        Self {
            rows,
            cols,
            delay,
            row_index: 0,
        }
    }
}

#[async_trait::async_trait]
impl<'a> DisplayDriver for MicrobitDriver<'a> {
    async fn render(&mut self, frame: &Frame) {
        let row = self.row_index;

        // Turn off all rows first
        for r in self.rows.iter_mut() {
            r.set_high();
        }

        // Set columns for this row
        let row_value = frame[row];
        for (col, c) in self.cols.iter_mut().enumerate() {
            if (row_value >> col) & 1 == 1 {
                c.set_low();
            } else {
                c.set_high();
            }
        }

        // Activate this row
        self.rows[row].set_low(); // active low
        self.delay.delay_us(300).await;

        self.row_index = (self.row_index + 1) % 5;
    }
}
