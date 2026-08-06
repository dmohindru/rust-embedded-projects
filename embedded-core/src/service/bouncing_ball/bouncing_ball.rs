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
    D: DrawTarget<Color = C> + Present,
{
    pub fn tick(&mut self) {
        self.bouncing_ball_core.tick();
    }

    pub async fn draw(&mut self) {
        // self.renderer
        //     .draw(self.bouncing_ball_core.snapshot(), &mut self.display)?;
        // self.display.present().await.unwrap();
        todo!()
    }
}
