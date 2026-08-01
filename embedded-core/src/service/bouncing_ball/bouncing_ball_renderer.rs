use embedded_graphics::{
    geometry::Point,
    pixelcolor::PixelColor,
    primitives::{Circle, Primitive, PrimitiveStyle},
    Drawable,
};
use embedded_graphics_core::draw_target::DrawTarget;

use crate::service::bouncing_ball::bouncing_ball_core::BouncingBallSnapshot;

pub struct BouncingBallRenderer<C>
where
    C: PixelColor,
{
    ball_color: C,
}

impl<C> BouncingBallRenderer<C>
where
    C: PixelColor,
{
    pub fn new(ball_color: C) -> Self {
        BouncingBallRenderer { ball_color }
    }

    pub fn draw<D>(&self, snapshot: BouncingBallSnapshot, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let fill_style = PrimitiveStyle::with_fill(self.ball_color);
        Circle::new(
            Point::new(snapshot.ball_coordinates.0.x, snapshot.ball_coordinates.0.y),
            snapshot.radius as u32,
        )
        .into_styled(fill_style)
        .draw(display)?;

        Circle::new(
            Point::new(snapshot.ball_coordinates.1.x, snapshot.ball_coordinates.1.y),
            snapshot.radius as u32,
        )
        .into_styled(fill_style)
        .draw(display)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::service::bouncing_ball::bouncing_ball_core::Ball;

    use super::*;
    use embedded_graphics::{geometry::Size, mock_display::MockDisplay, pixelcolor::BinaryColor};
    use embedded_graphics_simulator::{
        BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
    };

    #[test]
    #[ignore = "visual test only"]
    fn visual_display_test() {
        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
        let renderer = BouncingBallRenderer::<BinaryColor>::new(BinaryColor::On);
        let ball_one = Ball {
            x: 30,
            y: 30,
            x_dir: 1,
            y_dir: 1,
        };

        let ball_two = Ball {
            x: 90,
            y: 40,
            x_dir: 1,
            y_dir: 1,
        };

        let snapshot = BouncingBallSnapshot {
            ball_coordinates: (&ball_one, &ball_two),
            radius: 3,
        };

        renderer.draw(snapshot, &mut display).unwrap();
        let output_setting = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledWhite)
            .build();
        Window::new("Renderer Test", &output_setting).show_static(&display);
    }

    #[test]
    fn mock_display_test() {
        let mut display = MockDisplay::<BinaryColor>::new();

        let renderer = BouncingBallRenderer::<BinaryColor>::new(BinaryColor::On);
        let ball_one = Ball {
            x: 0,
            y: 0,
            x_dir: 1,
            y_dir: 1,
        };

        let ball_two = Ball {
            x: 4,
            y: 0,
            x_dir: 1,
            y_dir: 1,
        };

        let snapshot = BouncingBallSnapshot {
            ball_coordinates: (&ball_one, &ball_two),
            radius: 3,
        };

        renderer.draw(snapshot, &mut display).unwrap();
        display.assert_pattern(&[" #   # ", "### ###", " #   # "]);
    }
}
