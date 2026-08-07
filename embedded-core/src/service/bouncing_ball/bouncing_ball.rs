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
    pub fn new(
        display: D,
        radius: usize,
        step_size: usize,
        ball_color: C,
        background_color: C,
    ) -> Self {
        Self {
            bouncing_ball_core: BouncingBallCore::<WIDTH, HEIGHT>::new(radius, step_size),
            renderer: BouncingBallRenderer::<C>::new(ball_color, background_color),
            display: display,
        }
    }

    pub async fn update(&mut self) -> Result<(), D::PresentError> {
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

#[cfg(test)]
mod tests {

    use super::*;
    use embedded_graphics::{
        geometry::{OriginDimensions, Size},
        pixelcolor::BinaryColor,
    };
    use embedded_graphics_core::draw_target::DrawTarget;
    use embedded_graphics_simulator::{
        BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
    };

    pub struct SimulatorDisplayPresenter {
        display: SimulatorDisplay<BinaryColor>,
        window: Window,
    }

    impl SimulatorDisplayPresenter {
        pub fn new(width: u32, height: u32) -> Self {
            let display = SimulatorDisplay::<BinaryColor>::new(Size::new(width, height));

            let output_settings = OutputSettingsBuilder::new()
                .theme(BinaryColorTheme::OledWhite)
                .build();

            let window = Window::new("Bouncing Ball", &output_settings);
            Self { display, window }
        }
    }

    impl DrawTarget for SimulatorDisplayPresenter {
        type Color = BinaryColor;
        type Error = Infallible;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
        {
            self.display.draw_iter(pixels)
        }
    }

    impl OriginDimensions for SimulatorDisplayPresenter {
        fn size(&self) -> Size {
            Size::new(128, 64)
        }
    }

    impl Present for SimulatorDisplayPresenter {
        type PresentError = Infallible;

        async fn present(&mut self) -> Result<(), Self::PresentError> {
            self.window.update(&self.display);

            Ok(())
        }
    }

    #[tokio::test]
    #[ignore = "visual test only"]
    async fn visual_test_bouncing_ball_service() {
        let simulator_presenter = SimulatorDisplayPresenter::new(128, 64);
        let mut bouncing_ball = BouncingBall::<_, 128, 64, BinaryColor>::new(
            simulator_presenter,
            3,
            5,
            BinaryColor::On,
            BinaryColor::Off,
        );
        loop {
            bouncing_ball.update().await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        }
    }
}
