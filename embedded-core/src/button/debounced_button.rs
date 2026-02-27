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

#[cfg(test)]
mod test {
    use super::*;

    struct FakeDelay;
    impl DelayNs for FakeDelay {
        async fn delay_ns(&mut self, ns: u32) {}
    }

    #[tokio::test]
    async fn debounced_button_test() {
        use embedded_hal_mock::eh1::digital::Mock as PinMock;
        use embedded_hal_mock::eh1::digital::{Edge, State, Transaction};
        let expectations = [
            Transaction::wait_for_edge(Edge::Falling),
            Transaction::get(State::High),
            Transaction::wait_for_edge(Edge::Rising),
        ];
        let mock_pin = PinMock::new(&expectations);
        let mut button = DebouncedButton::new(mock_pin, FakeDelay, 10);
        let mut called = false;
        button
            .wait(|| async {
                // called = true;
            })
            .await;

        assert!(called);
    }
}
