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

#[cfg(test)]
impl<D, const WIDTH: usize, const HEIGHT: usize> BouncingBall<D, WIDTH, HEIGHT>
where
    D: DrawTarget + Present,
{
    pub fn set_circle_points(&mut self, circle_one_center: Point, circle_two_center: Point) {
        self.circle_one_center = circle_one_center;
        self.circle_two_center = circle_two_center;
    }

    pub fn get_circle_points(&self) -> (Point, Point) {
        (self.circle_one_center, self.circle_two_center)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_balls_in_right_direction() {
        todo!()
    }

    #[test]
    fn should_bounce_ball_off_top_and_bottom_edge() {
        todo!()
    }

    #[test]
    fn should_bounce_ball_off_left_and_right_edge() {
        todo!()
    }

    #[test]
    fn should_bounce_ball_off_top_left_and_bottom_right_corner() {
        todo!()
    }

    #[test]
    fn should_bounce_ball_off_top_right_and_bottom_left_corner() {
        todo!()
    }

    #[test]
    fn should_bounce_off_each_other_on_collision() {
        todo!()
    }

    #[test]
    fn should_display_animation() {
        todo!()
    }
}
