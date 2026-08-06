use core::convert::Infallible;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::PixelColor};

use crate::{
    display::Present,
    service::bouncing_ball::{bouncing_ball_core::BouncingBallCore, BouncingBallRenderer},
};

pub struct BouncingBall<D, const WIDTH: usize, const HEIGHT: usize, C: PixelColor>
where
    D: DrawTarget<Color = C> + Present,
{
    bouncing_ball_core: BouncingBallCore<WIDTH, HEIGHT>,
    renderer: BouncingBallRenderer<C>,
    display: D,
}

impl<D, const WIDTH: usize, const HEIGHT: usize, C: PixelColor> BouncingBall<D, WIDTH, HEIGHT, C>
where
    D: DrawTarget<Color = C, Error = Infallible> + Present,
{
    pub async fn update(&mut self) -> Result<(), <D as Present>::Error> {
        self.bouncing_ball_core.tick();

        // This operation cannot fail
        self.display
            .clear(*self.renderer.get_background_color())
            .unwrap();

        // This operation cannot fail
        self.renderer
            .draw(self.bouncing_ball_core.snapshot(), &mut self.display)
            .unwrap();

        self.display.present().await?;
        Ok(())
    }
}
