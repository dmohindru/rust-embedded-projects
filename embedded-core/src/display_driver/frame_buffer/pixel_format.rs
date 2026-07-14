use embedded_graphics::pixelcolor::BinaryColor;
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

pub struct BinaryPixelFormat;

impl PixelFormat for BinaryPixelFormat {
    type Color = BinaryColor;
    const BITS_PER_PIXEL: usize = 1;

    fn encode(color: Self::Color) -> u32 {
        match color {
            BinaryColor::On => 1,
            BinaryColor::Off => 0,
        }
    }

    fn decode(bits: u32) -> Self::Color {
        if bits == 1 {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }
}

#[cfg(test)]
mod binary_pixel_format_test {
    use super::*;

    #[test]
    fn should_encode_binary_color_on() {
        assert_eq!(1, BinaryPixelFormat::encode(BinaryColor::On));
    }

    #[test]
    fn should_encode_binary_color_off() {
        assert_eq!(0, BinaryPixelFormat::encode(BinaryColor::Off));
    }

    #[test]
    fn should_get_binary_color_on_from_encoded_value() {
        assert_eq!(BinaryColor::On, BinaryPixelFormat::decode(1));
    }

    #[test]
    fn should_get_binary_color_off_from_encoded_value() {
        assert_eq!(BinaryColor::Off, BinaryPixelFormat::decode(0));
    }
}
