use crate::frame::Frame;
use defmt::Format;
use heapless::Vec;

#[derive(Format, Clone, Copy)]
pub enum Rotation {
    ClockWise,
    AntiClockWise,
}

pub struct FrameCursorCircular<const N: usize, const R: usize, const C: usize> {
    frames: Vec<Frame<R, C>, N>,
    index: usize,
}

impl<const N: usize, const R: usize, const C: usize> FrameCursorCircular<N, R, C> {
    pub fn new(initial_frames: &[Frame<R, C>]) -> Self {
        let mut frames = Vec::new();

        for f in initial_frames {
            frames
                .push(*f)
                .expect("unable to insert frame into circular frame cursor");
        }

        Self { frames, index: 0 }
    }

    pub fn current_frame(&self) -> &Frame<R, C> {
        &self.frames[self.index]
    }

    pub fn move_index(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::ClockWise => {
                self.index = (self.index + 1) % N;
            }
            Rotation::AntiClockWise => {
                self.index = (self.index + N - 1) % N;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    static F1_DATA: [u32; 5] = [1, 1, 1, 1, 1];
    static F2_DATA: [u32; 5] = [2, 2, 2, 2, 2];
    static F3_DATA: [u32; 5] = [3, 3, 3, 3, 3];
    static F4_DATA: [u32; 5] = [4, 4, 4, 4, 4];
    fn frame(data: [u32; 5]) -> Frame<5, 5> {
        Frame::<5, 5>::new(data)
    }

    fn before_each() -> FrameCursorCircular<4, 5, 5> {
        let frames = [
            frame(F1_DATA),
            frame(F2_DATA),
            frame(F3_DATA),
            frame(F4_DATA),
        ];
        FrameCursorCircular::<4, 5, 5>::new(&frames)
    }

    #[test]
    fn should_return_first_frame() {
        let frame_cursor = before_each();
        assert_eq!(&frame(F1_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_move_frame_to_next_for_clockwise_circular_rotation() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Rotation::ClockWise);
        assert_eq!(&frame(F2_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_move_back_to_start_position_for_multiple_clockwise_circular_rotation() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Rotation::ClockWise);
        frame_cursor.move_index(Rotation::ClockWise);
        frame_cursor.move_index(Rotation::ClockWise);
        frame_cursor.move_index(Rotation::ClockWise);
        assert_eq!(&frame(F1_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_move_frame_to_previous_position_for_anticlockwise_circular_rotation() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Rotation::ClockWise);
        frame_cursor.move_index(Rotation::AntiClockWise);
        assert_eq!(&frame(F1_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_move_frame_to_last_position_for_multiple_anticlockwise_circular_rotation() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Rotation::ClockWise);
        frame_cursor.move_index(Rotation::AntiClockWise);
        frame_cursor.move_index(Rotation::AntiClockWise);
        assert_eq!(&frame(F4_DATA), frame_cursor.current_frame());
    }
}
