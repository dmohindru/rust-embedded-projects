mod frame;
mod frame_cursor;
mod frame_cursor_circular;
mod frame_decode;

use defmt::Format;
pub use frame::Frame;
pub use frame_cursor::FrameCursor;
pub use frame_cursor_circular::FrameCursorCircular;
pub use frame_decode::decode_frames;

#[derive(Format, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn toggle(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
