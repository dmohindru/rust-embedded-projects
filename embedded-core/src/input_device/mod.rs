pub mod button;
pub mod nunchuk;

trait Encode {
    fn encode(&self, out: &mut [u8]) -> usize;
}
