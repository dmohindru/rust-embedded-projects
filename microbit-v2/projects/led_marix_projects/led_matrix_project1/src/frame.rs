use heapless::Vec;

pub type Frame = [u8; 5];

#[derive(defmt::Format, Clone, Copy)]
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
