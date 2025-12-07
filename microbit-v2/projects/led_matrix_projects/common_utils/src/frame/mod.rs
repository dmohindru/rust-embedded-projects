pub struct Frame<const R: usize, const C: usize> {
    rows: [u32; R],
}

impl<const R: usize, const C: usize> Frame<R, C> {
    pub fn new(rows: [u32; R]) -> Self {
        const fn assert_max<const R: usize, const C: usize>() {
            if R > 32 || C > 32 {
                panic!("Max supported size of Row(R) and Column(C) is 32");
            }
        }

        assert_max::<R, C>();
        Self { rows }
    }

    #[inline]
    fn assert_idx(r: usize, c: usize) {
        if r >= R || c >= C {
            panic!(
                "Frame index out of bounds (r={}, c={}, R={}, C={})",
                r, c, R, C
            );
        }
    }

    pub fn set(&mut self, r: usize, c: usize, value: bool) {
        Self::assert_idx(r, c);

        if value {
            self.rows[r] |= 1 << c;
        } else {
            self.rows[r] &= !(1 << c);
        }
    }

    pub fn get(&self, r: usize, c: usize) -> bool {
        Self::assert_idx(r, c);
        (self.rows[r] >> c) & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::frame::Frame;
    static FRAME_DATA: [u32; 32] = [0x11111111; 32]; // [0001 0001 0001 0001 0001 0001 0001 0001; 32]

    #[test]
    #[should_panic(expected = "Max supported size of Row(R) and Column(C) is 32")]
    fn should_panic_when_frame_created_with_column_size_greater_than_32() {
        let frame_data: [u32; 33] = [0x1111; 33];
        Frame::<33, 12>::new(frame_data);
    }

    #[test]
    fn should_create_frame_when_row_column_within_bound() {
        let frame_data: [u32; 32] = [0x1111; 32];
        Frame::<32, 32>::new(frame_data);
    }

    #[test]
    #[should_panic(expected = "Frame index out of bounds (r=0, c=33, R=32, C=32)")]
    fn should_panic_when_setting_frame_column_out_of_bound() {
        let mut frame = Frame::<32, 32>::new(FRAME_DATA);
        frame.set(0, 33, false);
    }

    #[test]
    #[should_panic(expected = "Frame index out of bounds (r=32, c=32, R=32, C=32)")]
    fn should_panic_when_setting_frame_row_out_of_bound() {
        let mut frame = Frame::<32, 32>::new(FRAME_DATA);
        frame.set(32, 32, false);
    }

    #[test]
    fn should_set_frame_bit_when_row_column_within_bound() {
        let mut frame = Frame::<32, 32>::new(FRAME_DATA);
        frame.set(16, 15, true);
        let frame_bit = frame.get(16, 15);
        assert_eq!(true, frame_bit);
    }

    #[test]
    fn should_clear_frame_bit_when_row_column_within_bound() {
        let mut frame = Frame::<32, 32>::new(FRAME_DATA);
        frame.set(16, 16, false);
        let frame_bit = frame.get(16, 16);
        assert_eq!(false, frame_bit);
    }

    #[test]
    #[should_panic(expected = "Frame index out of bounds (r=32, c=16, R=32, C=32)")]
    fn should_panic_when_getting_frame_bit_out_of_bound() {
        let frame = Frame::<32, 32>::new(FRAME_DATA);
        frame.get(32, 16);
    }

    #[test]
    fn should_get_frame_bit_when_row_column_within_bound() {
        let frame = Frame::<32, 32>::new(FRAME_DATA);
        let frame_bit = frame.get(16, 16);
        assert_eq!(true, frame_bit);
    }
}
