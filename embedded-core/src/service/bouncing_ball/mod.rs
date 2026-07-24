use crate::display::Present;
use embedded_graphics::{geometry::Point, primitives::Circle};
use embedded_graphics_core::draw_target::DrawTarget;
pub struct BouncingBall<D, const WIDTH: usize, const HEIGHT: usize>
where
    D: DrawTarget + Present,
{
    display: D,
    radius: usize,
    circle_one_center: Point,
    circle_two_center: Point,
}

impl<D, const WIDTH: usize, const HEIGHT: usize> BouncingBall<D, WIDTH, HEIGHT>
where
    D: DrawTarget + Present,
{
    pub fn new(display: D, radius: usize) -> Self {
        let center_y: i32 = HEIGHT as i32 / 2;
        let center_one_x = WIDTH as i32 / 4;
        let center_two_x = center_one_x * 3;
        let circle_one_center = Point::new(center_one_x, center_y);
        let circle_two_center = Point::new(center_two_x, center_y);
        Self {
            display,
            radius,
            circle_one_center,
            circle_two_center,
        }
    }

    pub fn tick(&mut self) {}

    pub fn render(&mut self) -> Result<(), <D as Present>::Error> {
        todo!()
    }

    fn draw(&mut self) -> Result<(), <D as DrawTarget>::Error> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), <D as Present>::Error> {
        todo!()
    }
}
