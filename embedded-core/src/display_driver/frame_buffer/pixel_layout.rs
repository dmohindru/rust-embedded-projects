pub trait PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32);

    fn read_pixel(framebuffer: &[u8], width: usize, x: usize, y: usize) -> u32;
}

struct DisplayDimensions {
    x: usize,
    y: usize,
}

pub enum DisplaySize {
    Display128x64, // DA 12
    Display128x32, // DA 02
    Display96x16,  // DA 02
}

impl DisplaySize {
    fn screen_size(&self) -> DisplayDimensions {
        match self {
            DisplaySize::Display128x32 => DisplayDimensions { x: 32, y: 128 },
            DisplaySize::Display128x64 => DisplayDimensions { x: 64, y: 128 },
            DisplaySize::Display96x16 => DisplayDimensions { x: 16, y: 96 },
        }
    }
}

pub struct Ssd1306PixelLayout;

impl PixelLayout for Ssd1306PixelLayout {
    fn write_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, encoded: u32) {
        todo!()
    }

    fn read_pixel(framebuffer: &[u8], width: usize, x: usize, y: usize) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod ssd1306_tests {
    use super::*;
    type Point = (usize, usize);

    #[test]
    fn should_write_pixel_data_to_right_memory_location() {
        let mut frame_buffer: [u8; 1024] = [0; 1024];
        let width = 128;
        let rect_top_left: (Point, Point) = ((2, 2), (6, 6));
        let rect_top_right: (Point, Point) = ((124, 2), (126, 4));
        let rect_bottom_left: (Point, Point) = ((2, 60), (4, 62));
        let rect_bottom_right: (Point, Point) = ((122, 58), (126, 62));
        // Set pixels
        set_rectangle_pixels(&mut frame_buffer, width, rect_top_left.0, rect_top_left.1);
        set_rectangle_pixels(&mut frame_buffer, width, rect_top_right.0, rect_top_right.1);
        set_rectangle_pixels(
            &mut frame_buffer,
            width,
            rect_bottom_left.0,
            rect_bottom_left.1,
        );
        set_rectangle_pixels(
            &mut frame_buffer,
            width,
            rect_bottom_right.0,
            rect_bottom_right.1,
        );

        // Verify pixels
        verify_rectangle_pixels(
            &mut frame_buffer,
            width,
            rect_top_left.0,
            rect_top_left.1,
            1,
        );
        verify_rectangle_pixels(
            &mut frame_buffer,
            width,
            rect_top_right.0,
            rect_top_right.1,
            1,
        );
        verify_rectangle_pixels(
            &mut frame_buffer,
            width,
            rect_bottom_left.0,
            rect_bottom_left.1,
            1,
        );
        verify_rectangle_pixels(
            &mut frame_buffer,
            width,
            rect_bottom_right.0,
            rect_bottom_right.1,
            1,
        );

        // verify some random locations not being set
        assert_eq!(
            0,
            Ssd1306PixelLayout::read_pixel(&mut frame_buffer, width, 0, 0)
        );

        assert_eq!(
            0,
            Ssd1306PixelLayout::read_pixel(&mut frame_buffer, width, 127, 63)
        );

        assert_eq!(
            0,
            Ssd1306PixelLayout::read_pixel(&mut frame_buffer, width, 100, 100)
        );

        assert_eq!(
            0,
            Ssd1306PixelLayout::read_pixel(&mut frame_buffer, width, 50, 50)
        )
    }

    fn set_rectangle_pixels(
        frame_buffer: &mut [u8],
        width: usize,
        top_left: Point,
        bottom_right: Point,
    ) {
        for y in (top_left.0)..(bottom_right.0 + 1) {
            for x in (top_left.1)..(bottom_right.1 + 1) {
                Ssd1306PixelLayout::write_pixel(frame_buffer, width, x, y, 1);
            }
        }
    }

    fn verify_rectangle_pixels(
        frame_buffer: &mut [u8],
        width: usize,
        top_left: Point,
        bottom_right: Point,
        expected_value: u32,
    ) {
        for y in (top_left.0)..(bottom_right.0 + 1) {
            for x in (top_left.1)..(bottom_right.1 + 1) {
                assert_eq!(
                    expected_value,
                    Ssd1306PixelLayout::read_pixel(frame_buffer, width, x, y)
                );
            }
        }
    }
}
