use embassy_nrf::gpio::Input;
use embassy_time::Timer;

pub struct DebouncedButton {
    button: Input<'static>,
    debounce_ms: u64,
}

impl DebouncedButton {
    pub fn new(button: Input<'static>, debounce_ms: u64) -> Self {
        DebouncedButton {
            button,
            debounce_ms,
        }
    }

    pub async fn wait<F, Fut>(&mut self, mut on_press: F)
    where
        F: FnMut() -> Fut,
        Fut: core::future::Future<Output = ()>,
    {
        loop {
            self.button.wait_for_low().await;

            Timer::after_millis(self.debounce_ms).await;

            if self.button.is_low() {
                on_press().await;
            }

            // // Wait until released
            self.button.wait_for_high().await;
        }
    }
}
