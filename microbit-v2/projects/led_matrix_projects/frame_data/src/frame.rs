use defmt::Format;
use heapless::Vec;

pub type Frame = [u8; 5];

#[derive(Format, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct FrameData<const N: usize> {
    frames: Vec<Frame, N>,
    index: usize,
}

impl<const N: usize> FrameData<N> {
    pub fn new(initial_frames: &[Frame]) -> Self {
        let mut frames = Vec::new();

        for f in initial_frames {
            frames
                .push(*f)
                .expect("unable to insert frame into frame data");
        }
        let mid = frames.len() / 2;

        Self { frames, index: mid }
    }

    pub fn current_frame(&self) -> &Frame {
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
    // Forces std mode for the module
    extern crate std;
    use super::*;
    use std::dbg;
    const F1: Frame = [1, 1, 1, 1, 1];
    const F2: Frame = [2, 2, 2, 2, 2];
    const F3: Frame = [3, 3, 3, 3, 3];

    const FRAMES: [Frame; 3] = [F1, F2, F3];

    #[test]
    fn should_return_center_frame() {
        let frame_data = FrameData::<3>::new(&FRAMES);
        assert_eq!(&F2, frame_data.current_frame());
    }

    #[test]
    fn should_move_frame_to_left() {
        let mut frame_data = FrameData::<3>::new(&FRAMES);
        frame_data.move_index(Direction::Left);
        assert_eq!(&F1, frame_data.current_frame());
    }

    #[test]
    fn should_not_move_frame_past_leftmost_position() {
        let mut frame_data = FrameData::<3>::new(&FRAMES);
        frame_data.move_index(Direction::Left);
        frame_data.move_index(Direction::Left);
        assert_eq!(&F1, frame_data.current_frame());
    }

    #[test]
    fn should_move_frame_to_right() {
        let mut frame_data = FrameData::<3>::new(&FRAMES);
        frame_data.move_index(Direction::Right);
        assert_eq!(&F3, frame_data.current_frame());
    }

    #[test]
    fn should_not_move_frame_past_rightmost_position() {
        let mut frame_data = FrameData::<3>::new(&FRAMES);
        frame_data.move_index(Direction::Right);
        frame_data.move_index(Direction::Right);
        assert_eq!(&F3, frame_data.current_frame());
    }
}
