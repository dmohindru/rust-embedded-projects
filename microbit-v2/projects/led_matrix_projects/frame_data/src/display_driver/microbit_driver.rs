use crate::display_driver::{Delay, DisplayDriver, PinOutput};
use crate::frame::Frame;

#[cfg(test)]
use embedded_hal::digital::OutputPin;

#[cfg(not(test))]
use embassy_nrf::gpio::Output;

#[cfg(not(test))]
use embassy_time::{block_for, Duration as EmbassyDuration};

//
// PinOutput implementations
//

#[cfg(not(test))]
impl PinOutput for Output<'_> {
    fn set_high(&mut self) {
        self.set_high();
    }

    fn set_low(&mut self) {
        self.set_low();
    }
}

#[cfg(test)]
impl<P> PinOutput for P
where
    P: OutputPin,
{
    fn set_high(&mut self) {
        self.set_high().unwrap();
    }

    fn set_low(&mut self) {
        self.set_low().unwrap();
    }
}

//
// Delay implementations
//

#[cfg(not(test))]
pub struct EmbassyDelay;

#[cfg(not(test))]
impl Delay for EmbassyDelay {
    fn delay_us(&mut self, us: u32) {
        block_for(EmbassyDuration::from_micros(us as u64));
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct MockDelay {
    pub calls: usize,
}

#[cfg(test)]
impl Delay for MockDelay {
    fn delay_us(&mut self, _us: u32) {
        self.calls += 1;
    }
}

//
// MicrobitDriver
//

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

impl<'a> DisplayDriver for MicrobitDriver<'a> {
    fn render(&mut self, frame: &Frame) {
        let row = self.row_index;

        // Disable all rows first (active-low)
        for r in self.rows.iter_mut() {
            r.set_high();
        }

        // Set column states for this row
        let row_value = frame[row];
        for (col, c) in self.cols.iter_mut().enumerate() {
            if (row_value >> col) & 1 == 1 {
                c.set_low(); // LED ON
            } else {
                c.set_high(); // LED OFF
            }
        }

        // Activate selected row (active low)
        self.rows[row].set_low();

        // Hold for 300 µs
        self.delay.delay_us(300);

        // Move to next row
        self.row_index = (self.row_index + 1) % 5;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use embedded_hal_mock::eh0::pin::{Mock as PinMock, Transaction as PinTransaction};

//     fn mock_row_pins() -> [PinMock; 5] {
//         // Each row expects: set_high(), then set_low() ONLY for active row
//         [
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//         ]
//     }

//     fn mock_col_pins() -> [PinMock; 5] {
//         // Columns get many set_high/set_low calls depending on frame
//         [
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//             PinMock::new(&[]),
//         ]
//     }

//     fn wrap_mut_refs<T>(arr: &mut [T]) -> [&mut dyn PinOutput; 5]
//     where
//         T: PinOutput,
//     {
//         assert!(arr.len() == 5);
//         let (five, _) = arr.split_at_mut(5);

//         let [a, b, c, d, e] = five else {
//             unreachable!()
//         };
//         [a, b, c, d, e]
//     }

//     #[test]
//     fn test_render_first_row() {
//         // FRAME:
//         // Row 0 bits (10101) → columns LOW, HIGH, LOW, HIGH, LOW
//         let frame: Frame = [0b10101, 0, 0, 0, 0];

//         // rows + columns
//         let mut rows = mock_row_pins();
//         let mut cols = mock_col_pins();

//         let mut delay = MockDelay { calls: 0 };

//         // Convert to trait object slices
//         let row_refs = wrap_mut_refs(&mut rows[..]);
//         let col_refs = wrap_mut_refs(&mut cols[..]);

//         // Create driver
//         let mut driver = MicrobitDriver::new(row_refs, col_refs, &mut delay);

//         // ---- Call render ----
//         driver.render(&frame);

//         // ---- Validate delay was called ----
//         assert_eq!(delay.calls, 1, "delay must be called exactly once");

//         // ---- Validate row index advanced ----
//         assert_eq!(driver.row_index, 1);

//         // ---- Validate rows: all high then row0 low ----
//         // row0 should receive high then low
//         // others should receive only high

//         // Extract transactions to verify behavior
//         for (i, row_pin) in rows.into_iter().enumerate() {
//             // let actions = row_pin.done();

//             if i == 0 {
//                 // first row: expected [set_high, set_low]
//                 // assert_eq!(actions.len(), 2, "row0 should have 2 actions");
//             } else {
//                 // inactive rows: [set_high]
//                 // assert_eq!(actions.len(), 1, "inactive rows should have 1 action");
//             }
//         }

//         // ---- Validate columns toggled according to frame bits ----
//         let expected_bits = [1, 0, 1, 0, 1];

//         for (i, col_pin) in cols.into_iter().enumerate() {
//             // let actions = col_pin.done();

//             // For each column: exactly one call (set_low or set_high)
//             // assert_eq!(actions.len(), 1);

//             // match expected_bits[i] {
//             //     1 => assert_eq!(actions[0], PinTransaction::set_low()), // LED ON
//             //     0 => assert_eq!(actions[0], PinTransaction::set_high()), // LED OFF
//             //     _ => unreachable!(),
//             // }
//         }
//     }

//     #[test]
//     fn test_row_index_wraps() {
//         let frame: Frame = [0, 0, 0, 0, 0];

//         let mut rows = mock_row_pins();
//         let mut cols = mock_col_pins();
//         let mut delay = MockDelay { calls: 0 };

//         let row_refs = wrap_mut_refs(&mut rows);
//         let col_refs = wrap_mut_refs(&mut cols);

//         let mut driver = MicrobitDriver::new(row_refs, col_refs, &mut delay);

//         // Move to last row (index 4)
//         driver.row_index = 4;

//         driver.render(&frame);

//         assert_eq!(driver.row_index, 0, "row_index should wrap back to 0");
//         assert_eq!(delay.calls, 1);
//     }
// }
