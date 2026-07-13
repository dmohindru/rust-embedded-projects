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
pub trait PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32);

    fn read_pixel(framebuffer: &[u8], width: usize, x: usize, y: usize) -> u32;
}

pub trait PixelFormat {
    /// Number of significant bits returned by `encode()`.
    const BITS_PER_PIXEL: usize;

    type Color;

    /// Encodes a color into an integer.
    ///
    /// The encoded value occupies the least significant
    /// `BITS_PER_PIXEL` bits of the returned `u32`.
    /// All higher bits must be zero.
    fn encode(color: Self::Color) -> u32;

    fn decode(bits: u32) -> Self::Color;
}
/*
| Format | Bits |
| ------ | ---- |
| Binary | 1    |
| Gray2  | 2    |
| Gray4  | 4    |
| Gray8  | 8    |
| RGB565 | 16   |
| RGB666 | 18   |
| RGB888 | 24   |

PixelLayout depends only on:

- BITS_PER_PIXEL
- Encoded integer
*/
