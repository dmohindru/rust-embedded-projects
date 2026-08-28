use crate::input_device::Encode;
pub enum Command {
    BlackInitFirstRegister,
    BlackInitSecondRegister,
    WhiteInitFirstRegister,
    WhiteInitSecondRegister,
}

impl Encode for Command {
    fn encode(&self, out: &mut [u8]) -> usize {
        let length: usize = match self {
            Command::BlackInitFirstRegister => {
                out[0] = 0xF0;
                out[1] = 0x55;
                2
            }
            Command::BlackInitSecondRegister => {
                out[0] = 0xFB;
                out[1] = 0x00;
                2
            }
            Command::WhiteInitFirstRegister => {
                out[0] = 0x40;
                out[1] = 0x00;
                2
            }
            Command::WhiteInitSecondRegister => {
                out[0] = 0x00;
                1
            }
        };
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_provide_white_nunchuk_first_register_initialization_bytes_encoding() {
        let mut out: [u8; 4] = [0; 4];
        let len = Command::BlackInitFirstRegister.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xF0, out[0]);

        // Second byte
        assert_eq!(0x55, out[1]);
    }

    #[test]
    fn should_provide_white_nunchuk_second_register_initialization_bytes_encoding() {
        let mut out: [u8; 4] = [0; 4];
        let len = Command::BlackInitSecondRegister.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xFB, out[0]);

        // Second byte
        assert_eq!(0x00, out[1]);
    }

    #[test]
    fn should_provide_black_nunchuk_first_register_initialization_bytes_encoding() {
        let mut out: [u8; 4] = [0; 4];
        let len = Command::WhiteInitFirstRegister.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0x40, out[0]);

        // Second byte
        assert_eq!(0x00, out[1]);
    }

    #[test]
    fn should_provide_black_nunchuk_second_register_initialization_bytes_encoding() {
        let mut out: [u8; 4] = [0; 4];
        let len = Command::WhiteInitSecondRegister.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0x00, out[0]);
    }
}
