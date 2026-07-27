use embedded_graphics::draw_target::DrawTarget;

use crate::{
    display::Present,
    service::bouncing_ball::{bouncing_ball_core::BouncingBallCore, BouncingBallRenderer},
};

pub struct BouncingBall<D, const WIDTH: usize, const HEIGHT: usize>
where
    D: DrawTarget + Present,
{
    bouncing_ball_core: BouncingBallCore<WIDTH, HEIGHT>,
    renderer: BouncingBallRenderer,
    display: D,
}

impl<D, const WIDTH: usize, const HEIGHT: usize> BouncingBall<D, WIDTH, HEIGHT>
where
    D: DrawTarget + Present,
{
    pub fn tick(&mut self) {
        self.bouncing_ball_core.tick();
        todo!()
    }

    pub fn draw(&mut self) {
        self.renderer
            .draw(self.bouncing_ball_core.snapshot(), &mut self.display);
        //self.display.present().unwrap();
        todo!()
    }
}
