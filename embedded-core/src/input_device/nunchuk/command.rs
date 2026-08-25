use crate::input_device::Encode;
pub enum Command {
    BlackInitFirstRegister,
    BlackInitSecondRegister,
    WhiteInitFirstRegister,
    WhileInitSecondRegister,
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
            Command::WhileInitSecondRegister => {
                out[0] = 0x00;
                1
            }
        };
        length
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_provide_white_nunchuk_first_register_initialization_bytes_encoding() {
        todo!()
    }

    #[test]
    fn should_provide_white_nunchuk_second_register_initialization_bytes_encoding() {
        todo!()
    }

    #[test]
    fn should_provide_black_nunchuk_first_register_initialization_bytes_encoding() {
        todo!()
    }

    #[test]
    fn should_provide_black_nunchuk_second_register_initialization_bytes_encoding() {
        todo!()
    }
}
