use embedded_hal::digital::InputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::digital::Wait;
pub struct DebouncedButton<P, D> {
    button: P,
    delay: D,
    debounce_ms: u32,
}

impl<P, D> DebouncedButton<P, D>
where
    P: InputPin + Wait,
    D: DelayNs,
{
    pub fn new(button: P, delay: D, debounce_ms: u32) -> Self {
        Self {
            button,
            delay,
            debounce_ms,
        }
    }

    pub async fn wait<F, Fut>(&mut self, mut on_press: F)
    where
        F: FnMut() -> Fut,
        Fut: core::future::Future<Output = ()>,
    {
        loop {
            self.button.wait_for_low().await.unwrap();

            self.delay.delay_ms(self.debounce_ms).await;

            if self.button.is_low().unwrap() {
                on_press().await;
            }

            self.button.wait_for_high().await.unwrap();
        }
    }
}
