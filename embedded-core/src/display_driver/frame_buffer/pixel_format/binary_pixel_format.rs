use crate::display_driver::frame_buffer::pixel_format::PixelFormat;
use embedded_graphics::pixelcolor::BinaryColor;

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
