mod binary_pixel_format;
mod rgb565_pixel_format;

pub use binary_pixel_format::BinaryPixelFormat;
use embedded_graphics::pixelcolor::PixelColor;
pub use rgb565_pixel_format::Rgb565PixelFormat;

pub trait PixelFormat {
    /// Number of significant bits returned by `encode()`.
    const BITS_PER_PIXEL: usize;

    type Color: PixelColor;

    /// Encodes a color into an integer.
    ///
    /// The encoded value occupies the least significant
    /// `BITS_PER_PIXEL` bits of the returned `u32`.
    /// All higher bits must be zero.
    fn encode(color: Self::Color) -> u32;

    fn decode(bits: u32) -> Self::Color;
}
