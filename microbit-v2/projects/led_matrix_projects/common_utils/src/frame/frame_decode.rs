use crate::frame::Frame;

pub fn decode_frames<const N: usize, const R: usize, const C: usize>(
    data: &[u8],
) -> [Frame<R, C>; N] {
    assert!(
        data.len() == N * R,
        "Invalid frame data length: {} bytes for {} frames of {} rows",
        data.len(),
        N,
        R
    );

    core::array::from_fn(|i| {
        let offset = i * R;
        let mut rows = [0u32; R];

        for r in 0..R {
            rows[r] = data[offset + r] as u32;
        }

        Frame::<R, C>::new(rows)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_invalid_data() -> [u8; 9] {
        let data: [u8; 9] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        data
    }

    fn get_valid_data() -> [u8; 10] {
        let data: [u8; 10] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        data
    }

    #[test]
    #[should_panic(expected = "Invalid frame data length: 9 bytes for 2 frames of 5 rows")]
    fn decode_frames_should_panic_when_input_data_not_result_in_proper_frame_count() {
        let data = get_invalid_data();
        decode_frames::<2, 5, 5>(data.as_slice());
    }

    #[test]
    fn decoded_frames_should_return_array_with_right_number_frame_count() {
        let data = get_valid_data();
        let frames = decode_frames::<2, 5, 5>(data.as_slice());
        let expected_frame: [Frame<5, 5>; 2] = [
            Frame::new([0x00, 0x01, 0x02, 0x03, 0x04]),
            Frame::new([0x05, 0x06, 0x07, 0x08, 0x09]),
        ];

        assert_eq!(expected_frame, frames);
    }
}
