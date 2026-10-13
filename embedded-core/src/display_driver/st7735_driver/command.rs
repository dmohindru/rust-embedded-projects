use crate::display_driver::Encode;

pub enum Command {
    SWRESET,
    SLPOUT,
    FRMCTR1([u8; 3]),
}

impl Encode for Command {
    fn encode(&self, out: &mut [u8]) -> usize {
        let length: usize = match self {
            Command::SWRESET => {
                out[0] = 0x01;
                1
            }
            Command::SLPOUT => {
                out[0] = 0x11;
                1
            }
            Command::FRMCTR1(parameters) => {
                out[0] = 0xB1;
                out[1] = parameters[0] & 0x0F;
                out[2] = parameters[1] & 0x3F;
                out[3] = parameters[2] & 0x3F;
                4
            }
        };
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_provide_encoding_for_swreset_command() {
        let command = Command::SWRESET;
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        assert_eq!(0x01, out[0]);
    }

    #[test]
    fn should_provide_encoding_for_slpout_command() {
        let command = Command::SLPOUT;
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        assert_eq!(0x11, out[0]);
    }

    #[test]
    fn should_provide_encoding_for_frmctr1_command() {
        let command = Command::FRMCTR1([0xFF, 0xFF, 0xFF]);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(4, len);
        assert_eq!(0xB1, out[0]);
        assert_eq!(0x0F, out[1]);
        assert_eq!(0x3F, out[2]);
        assert_eq!(0x3F, out[3]);
    }
}
