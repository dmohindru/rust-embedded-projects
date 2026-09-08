use core::cell::Cell;
use embedded_hal_async::delay::DelayNs;
use tokio::time::{sleep, Duration};

pub struct FakeDelay {
    pub calls: Cell<u32>,
}

impl FakeDelay {
    pub fn new() -> Self {
        Self {
            calls: Cell::new(0),
        }
    }
}

impl DelayNs for FakeDelay {
    async fn delay_ns(&mut self, us: u32) {
        self.calls.set(self.calls.get() + 1);
        sleep(Duration::from_micros(us as u64)).await;
    }

    async fn delay_ms(&mut self, ms: u32) {
        self.delay_ns(ms).await;
    }
}
