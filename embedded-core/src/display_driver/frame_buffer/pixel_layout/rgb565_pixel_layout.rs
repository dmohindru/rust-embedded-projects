use crate::display_driver::frame_buffer::PixelLayout;

pub struct Rgb565PixelLayout;

impl PixelLayout for Rgb565PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32) {
        assert!(
            x < width,
            "Invalid coordinates, supplied x coordinate don't align with width param"
        );
        let bytes_per_pixel: usize = 2;
        let offset = (width * y + x) * bytes_per_pixel;
        let high_byte: u8 = (encoded >> 8) as u8 & 0xFF;
        let low_byte: u8 = encoded as u8 & 0xFF;
        framebuffer[offset] = high_byte;
        framebuffer[offset + 1] = low_byte;
    }

    fn read_pixel(_framebuffer: &[u8], _width: usize, _x: usize, _y: usize) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::display_driver::frame_buffer::{pixel_layout::Rgb565PixelLayout, PixelLayout};

    #[test]
    fn should_write_pixel_data_to_write_memory_location() {
        let width = 128;
        // let height = 64;
        // buffer size = 128 X 64 X 2 = 16384
        let mut framebuffer: [u8; 16384] = [0; 16384];
        let encoded_color: u32 = 0b11100_110111_00111;
        // Coordinates (0, 0)
        Rgb565PixelLayout::write_pixel(&mut framebuffer, width, 0, 0, encoded_color);
        assert_eq!(0b1110_0110, framebuffer[0]);
        assert_eq!(0b1110_0111, framebuffer[1]);

        // Coordinates (127, 0)
        Rgb565PixelLayout::write_pixel(&mut framebuffer, width, 127, 0, encoded_color);
        let offset: usize = 127 * 2;
        assert_eq!(0b1110_0110, framebuffer[offset]);
        assert_eq!(0b1110_0111, framebuffer[offset + 1]);

        // Coordinate (0, 1)
        Rgb565PixelLayout::write_pixel(&mut framebuffer, width, 0, 1, encoded_color);
        let offset: usize = (128 * 1 + 0) * 2;
        assert_eq!(0b1110_0110, framebuffer[offset]);
        assert_eq!(0b1110_0111, framebuffer[offset + 1]);

        // Coordinate (127, 63)
        Rgb565PixelLayout::write_pixel(&mut framebuffer, width, 127, 63, encoded_color);
        let offset: usize = (128 * 63 + 127) * 2;
        assert_eq!(0b1110_0110, framebuffer[offset]);
        assert_eq!(0b1110_0111, framebuffer[offset + 1]);

        // Coordinate (0, 63)
        Rgb565PixelLayout::write_pixel(&mut framebuffer, width, 0, 63, encoded_color);
        let offset: usize = (128 * 63 + 0) * 2;
        assert_eq!(0b1110_0110, framebuffer[offset]);
        assert_eq!(0b1110_0111, framebuffer[offset + 1]);

        // Coordinates (30, 30)
        Rgb565PixelLayout::write_pixel(&mut framebuffer, width, 30, 20, encoded_color);
        let offset: usize = (128 * 20 + 30) * 2;
        assert_eq!(0b1110_0110, framebuffer[offset]);
        assert_eq!(0b1110_0111, framebuffer[offset + 1]);
    }

    #[test]
    #[should_panic(
        expected = "Invalid coordinates, supplied x coordinate don't align with width param"
    )]
    fn should_panic_if_coordinates_do_not_align_with_supplied_width_param() {
        let width = 128;
        // let height = 64;
        // buffer size = 128 X 64 X 2 = 16384
        let mut framebuffer: [u8; 16384] = [0; 16384];
        let encoded_color: u32 = 0b11111_000111_000111;

        Rgb565PixelLayout::write_pixel(&mut framebuffer, width, 128, 0, encoded_color);
    }
}
