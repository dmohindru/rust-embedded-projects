use embedded_graphics_core::draw_target::DrawTarget;

use crate::service::bouncing_ball::bouncing_ball_core::BouncingBallSnapshot;

pub struct BouncingBallRenderer;

impl BouncingBallRenderer {
    pub fn draw<D>(&self, snapshot: BouncingBallSnapshot, display: &mut D)
    where
        D: DrawTarget,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn draw_test() {
        todo!()
    }
}
