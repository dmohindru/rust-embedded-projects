pub trait PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32);

    fn read_pixel(framebuffer: &[u8], width: usize, x: usize, y: usize) -> u32;
}

pub struct Ssd1306PixelLayout;

impl PixelLayout for Ssd1306PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32) {
        let page_size: usize = 8;
        let page_num = y / page_size;
        let byte_num = page_num * width + x;
        let bit_num = y % page_size;
        let mut byte = framebuffer[byte_num];
        byte = byte | (0x01 & (encoded as u8)) << bit_num;
        framebuffer[byte_num] = byte;
    }

    fn read_pixel(_framebuffer: &[u8], _width: usize, _x: usize, _y: usize) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod ssd1306_tests {
    use super::*;

    #[test]
    fn should_write_pixel_data_to_right_memory_location() {
        let mut framebuffer: [u8; 1024] = [0; 1024];
        let width = 128;
        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 0, 0, 1);
        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 0, 2, 1);
        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 0, 4, 1);
        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 0, 6, 1);
        let byte = framebuffer[0];
        assert_eq!(0x55, byte);

        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 127, 57, 1);
        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 127, 59, 1);
        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 127, 61, 1);
        Ssd1306PixelLayout::write_pixel(&mut framebuffer, width, 127, 63, 1);
        let byte = framebuffer[1023];
        assert_eq!(0xAA, byte);

        assert_eq!(0x00, framebuffer[1]);
        assert_eq!(0x00, framebuffer[1022]);
        assert_eq!(0x00, framebuffer[100]);
    }
}
