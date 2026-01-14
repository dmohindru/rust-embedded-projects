mod frame;
mod frame_cursor;
mod frame_cursor_circular;
use defmt::Format;
pub use frame::Frame;
pub use frame_cursor::FrameCursor;
pub use frame_cursor_circular::FrameCursorCircular;

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
