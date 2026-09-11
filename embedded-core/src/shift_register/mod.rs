mod hc165;
mod hc595;

pub use hc165::Hc165;
pub use hc595::{Error, Hc595};

#[allow(async_fn_in_trait)]
pub trait SerialInput {
    type Error;
    async fn read<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;
}
