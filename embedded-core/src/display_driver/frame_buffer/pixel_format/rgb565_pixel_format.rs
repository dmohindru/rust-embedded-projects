use crate::display_driver::frame_buffer::pixel_format::PixelFormat;
use embedded_graphics::pixelcolor::{raw::ToBytes, Rgb565};

pub struct Rgb565PixelFormat;

impl PixelFormat for Rgb565PixelFormat {
    type Color = Rgb565;
    const BITS_PER_PIXEL: usize = 16;

    fn encode(color: Self::Color) -> u32 {
        let [high, low] = color.to_be_bytes();
        ((high as u16) << 8 | low as u16) as u32
    }

    fn decode(bits: u32) -> Self::Color {
        let red_bits = ((bits >> 11) & 0x1F) as u8;
        let green_bits = ((bits >> 5) & 0x3F) as u8;
        let blue_bits = (bits & 0x1F) as u8;
        Rgb565::new(red_bits, green_bits, blue_bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_rbg565_color_red_bits() {
        let color = Rgb565::new(0xFF, 0x00, 0x00);
        let encoded_value = Rgb565PixelFormat::encode(color);
        assert_eq!(0xF800, encoded_value);
    }

    #[test]
    fn should_encode_rbg565_color_green_bits() {
        let color = Rgb565::new(0x00, 0xFF, 0x00);
        let encoded_value = Rgb565PixelFormat::encode(color);
        assert_eq!(0x07E0, encoded_value);
    }

    #[test]
    fn should_encode_rbg565_color_blue_bits() {
        let color = Rgb565::new(0x00, 0x00, 0xFF);
        let encoded_value = Rgb565PixelFormat::encode(color);
        assert_eq!(0x001F, encoded_value);
    }

    #[test]
    fn should_decode_red_color_bits() {
        let encoded_red: u32 = 0b11111_000000_00000;
        let decoded_red = Rgb565PixelFormat::decode(encoded_red);
        assert_eq!(decoded_red, Rgb565::new(0x1F, 0x00, 0x00));
    }

    #[test]
    fn should_decode_green_color_bits() {
        let encoded_green: u32 = 0b00000_111111_00000;
        let decoded_green = Rgb565PixelFormat::decode(encoded_green);
        assert_eq!(decoded_green, Rgb565::new(0x00, 0x3F, 0x00));
    }

    #[test]
    fn should_decode_blue_color_bits() {
        let encoded_blue: u32 = 0b00000_000000_11111;
        let decoded_blue = Rgb565PixelFormat::decode(encoded_blue);
        assert_eq!(decoded_blue, Rgb565::new(0x00, 0x00, 0x1F));
    }
}
