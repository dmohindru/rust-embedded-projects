use crate::frame::{Direction, Frame};
use heapless::Vec;

pub struct FrameCursor<const N: usize, const R: usize, const C: usize> {
    frames: Vec<Frame<R, C>, N>,
    index: usize,
}

impl<const N: usize, const R: usize, const C: usize> FrameCursor<N, R, C> {
    pub fn new(initial_frames: &[Frame<R, C>]) -> Self {
        let mut frames = Vec::new();

        for f in initial_frames {
            frames
                .push(*f)
                .expect("unable to insert frame into frame data");
        }
        let mid = frames.len() / 2;

        Self { frames, index: mid }
    }

    pub fn current_frame(&self) -> &Frame<R, C> {
        &self.frames[self.index]
    }

    pub fn move_index(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            Direction::Right => {
                if self.index < self.frames.len() - 1 {
                    self.index += 1;
                }
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
    fn frame(data: [u32; 5]) -> Frame<5, 5> {
        Frame::<5, 5>::new(data)
    }

    fn before_each() -> FrameCursor<3, 5, 5> {
        let frames = [frame(F1_DATA), frame(F2_DATA), frame(F3_DATA)];
        FrameCursor::<3, 5, 5>::new(&frames)
    }

    #[test]
    fn should_return_center_frame() {
        let frame_cursor = before_each();
        assert_eq!(&frame(F2_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_move_frame_to_left() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Direction::Left);
        assert_eq!(&frame(F1_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_not_move_frame_past_leftmost_position() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Direction::Left);
        frame_cursor.move_index(Direction::Left);
        assert_eq!(&frame(F1_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_move_frame_to_right() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Direction::Right);
        assert_eq!(&frame(F3_DATA), frame_cursor.current_frame());
    }

    #[test]
    fn should_not_move_frame_past_rightmost_position() {
        let mut frame_cursor = before_each();
        frame_cursor.move_index(Direction::Right);
        frame_cursor.move_index(Direction::Right);
        assert_eq!(&frame(F3_DATA), frame_cursor.current_frame());
    }
}
