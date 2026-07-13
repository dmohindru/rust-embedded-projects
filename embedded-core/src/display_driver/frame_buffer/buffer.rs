use crate::display_driver::frame_buffer::{PixelFormat, PixelLayout};
use core::marker::PhantomData;
pub struct FrameBuffer<const WIDTH: usize, const HEIGHT: usize, const BYTES: usize, L, P>
where
    L: PixelLayout,
    P: PixelFormat,
{
    pixels: [u8; BYTES],
    _marker: PhantomData<(L, P)>,
}

impl<const WIDTH: usize, const HEIGHT: usize, const BYTES: usize, L, P>
    FrameBuffer<WIDTH, HEIGHT, BYTES, L, P>
where
    L: PixelLayout,
    P: PixelFormat,
{
    pub fn new() -> Self {
        Self {
            pixels: [0; BYTES],
            _marker: PhantomData,
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: P::Color) {
        let encoded = P::encode(color);

        L::write_pixel(&mut self.pixels, WIDTH, x, y, encoded);
    }

    pub fn pixel(&self, x: usize, y: usize) -> P::Color {
        let encoded = L::read_pixel(&self.pixels, WIDTH, x, y);

        P::decode(encoded)
    }
}
