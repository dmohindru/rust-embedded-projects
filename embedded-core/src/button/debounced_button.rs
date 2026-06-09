use embedded_hal::digital::InputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::digital::Wait;

pub enum ActiveLevel {
    High,
    Low,
}

pub struct DebouncedButton<P, D> {
    button: P,
    delay: D,
    debounce_ms: u32,
    active_level: ActiveLevel,
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
            active_level: ActiveLevel::Low,
        }
    }

    pub fn with_active_level(mut self, active_level: ActiveLevel) -> Self {
        self.active_level = active_level;
        self
    }

    pub async fn wait<F, Fut>(&mut self, mut on_press: F)
    where
        F: FnMut() -> Fut,
        Fut: core::future::Future<Output = ()>,
    {
        loop {
            self.wait_once(&mut on_press).await;
        }
    }

    pub async fn wait_once<F, Fut>(&mut self, on_press: &mut F)
    where
        F: FnMut() -> Fut,
        Fut: core::future::Future<Output = ()>,
    {
        self.wait_for_active().await.unwrap();
        self.delay.delay_ms(self.debounce_ms).await;

        if self.is_active().unwrap() {
            on_press().await;
        }

        self.wait_for_not_active().await.unwrap();
    }

    async fn wait_for_active(&mut self) -> Result<(), P::Error> {
        match self.active_level {
            ActiveLevel::High => self.button.wait_for_high().await,
            ActiveLevel::Low => self.button.wait_for_low().await,
        }
    }

    async fn wait_for_not_active(&mut self) -> Result<(), P::Error> {
        match self.active_level {
            ActiveLevel::High => self.button.wait_for_low().await,
            ActiveLevel::Low => self.button.wait_for_high().await,
        }
    }

    fn is_active(&mut self) -> Result<bool, P::Error> {
        match self.active_level {
            ActiveLevel::High => self.button.is_high(),
            ActiveLevel::Low => self.button.is_low(),
        }
    }
}

#[cfg(test)]
impl<P, D> DebouncedButton<P, D> {
    pub fn free(self) -> (P, D) {
        (self.button, self.delay)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::cell::Cell;
    use embedded_hal_mock::eh1::digital::Mock as PinMock;
    use embedded_hal_mock::eh1::digital::{State, Transaction};

    struct FakeDelay;

    impl DelayNs for FakeDelay {
        async fn delay_ns(&mut self, _ns: u32) {}
    }

    #[tokio::test]
    async fn active_low_debounced_button_test() {
        let expectations = [
            Transaction::wait_for_state(State::Low),
            Transaction::get(State::Low),
            Transaction::wait_for_state(State::High),
        ];

        let mock_pin = PinMock::new(&expectations);
        let mut button = DebouncedButton::new(mock_pin, FakeDelay, 10);

        let called = Cell::new(false);

        let mut callback = || async {
            called.set(true);
        };

        button.wait_once(&mut callback).await;

        let (mut mock_pin, _) = button.free();
        mock_pin.done();

        assert!(called.get());
    }

    #[tokio::test]
    async fn active_high_debounced_button_test() {
        let expectations = [
            Transaction::wait_for_state(State::High),
            Transaction::get(State::High),
            Transaction::wait_for_state(State::Low),
        ];

        let mock_pin = PinMock::new(&expectations);
        let mut button =
            DebouncedButton::new(mock_pin, FakeDelay, 10).with_active_level(ActiveLevel::High);

        let called = Cell::new(false);

        let mut callback = || async {
            called.set(true);
        };

        button.wait_once(&mut callback).await;

        let (mut mock_pin, _) = button.free();
        mock_pin.done();

        assert!(called.get());
    }
}
