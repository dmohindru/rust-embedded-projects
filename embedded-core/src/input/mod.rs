use crate::input::{input_description::InputDescriptor, input_state::InputState};

mod input_description;
mod input_state;

/// Futures are intentionally not required to implement `Send`,
/// as this crate targets `no_std` embedded environments where
/// executors are typically single-threaded.
#[allow(async_fn_in_trait)]
pub trait InputDevice {
    type Error;
    /// Communicates with hardware and updates state
    async fn poll(&mut self) -> Result<(), Self::Error>;
    /// Describes what the device provides
    fn descriptor(&self) -> &InputDescriptor;
    /// Provides the latest sampled values
    fn state(&self) -> &InputState;
}
