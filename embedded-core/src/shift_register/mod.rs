mod hc165;
mod hc595;

pub use hc165::{Error as ErrorHC165, Hc165};
pub use hc595::{Error, Hc595};

#[allow(async_fn_in_trait)]
pub trait SerialInput<const N: usize> {
    type Error;
    async fn read(&mut self) -> Result<[u8; N], Self::Error>;
}
