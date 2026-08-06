/// Async presentation of the framebuffer.
///
/// Futures are intentionally not required to implement `Send`,
/// as this crate targets `no_std` embedded environments where
/// executors are typically single-threaded.
#[allow(async_fn_in_trait)]
pub trait Present {
    type Error;

    async fn present(&mut self) -> Result<(), Self::Error>;
}
