pub mod button;
pub mod make_code_input;
pub mod nunchuk;

trait Encode {
    fn encode(&self, out: &mut [u8]) -> usize;
}
