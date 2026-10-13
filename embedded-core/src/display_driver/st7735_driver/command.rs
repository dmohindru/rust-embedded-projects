use crate::display_driver::Encode;

pub enum Command {
    SWRESET,
    SLPOUT,
    FRMCTR1([u8; 3]),
    FRMCTR2([u8; 3]),
    FRMCTR3([u8; 6]),
    INVCTR(u8),
    PWCTR1([u8; 3]),
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
            Command::FRMCTR2(parameters) => {
                out[0] = 0xB2;
                out[1] = parameters[0] & 0x0F;
                out[2] = parameters[1] & 0x3F;
                out[3] = parameters[2] & 0x3F;
                4
            }
            Command::FRMCTR3(parameters) => {
                out[0] = 0xB3;
                out[1] = parameters[0] & 0x0F;
                out[2] = parameters[1] & 0x3F;
                out[3] = parameters[2] & 0x3F;
                out[4] = parameters[0] & 0x0F;
                out[5] = parameters[1] & 0x3F;
                out[6] = parameters[2] & 0x3F;
                7
            }
            Command::INVCTR(parameter) => {
                out[0] = 0xB4;
                out[1] = *parameter & 0x07;
                2
            }
            Command::PWCTR1(parameters) => {
                out[0] = 0xC0;
                out[1] = parameters[0];
                out[2] = parameters[1] & 0x1F;
                let parameter3 = parameters[2] << 6;
                out[3] = (parameters[2] << 6) | 0x04;
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

    #[test]
    fn should_provide_encoding_for_frmctr2_command() {
        let command = Command::FRMCTR2([0xFF, 0xFF, 0xFF]);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(4, len);
        assert_eq!(0xB2, out[0]);
        assert_eq!(0x0F, out[1]);
        assert_eq!(0x3F, out[2]);
        assert_eq!(0x3F, out[3]);
    }

    #[test]
    fn should_provide_encoding_for_frmctr3_command() {
        let command = Command::FRMCTR3([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        let mut out: [u8; 7] = [0; 7];
        let len = command.encode(&mut out);
        assert_eq!(7, len);
        assert_eq!(0xB3, out[0]);
        assert_eq!(0x0F, out[1]);
        assert_eq!(0x3F, out[2]);
        assert_eq!(0x3F, out[3]);
        assert_eq!(0x0F, out[4]);
        assert_eq!(0x3F, out[5]);
        assert_eq!(0x3F, out[6]);
    }

    #[test]
    fn should_provide_encoding_for_invctr_command() {
        let command = Command::INVCTR(0xFF);
        let mut out: [u8; 2] = [0; 2];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        assert_eq!(0xB4, out[0]);
        assert_eq!(0x07, out[1]);
    }

    #[test]
    fn should_provide_encoding_for_pwctr1_command() {
        let command = Command::PWCTR1([0xFF, 0xFF, 0xFF]);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(4, len);
        assert_eq!(0xC0, out[0]);
        assert_eq!(0xFF, out[1]);
        assert_eq!(0x1F, out[2]);
        assert_eq!(0xC4, out[3]);
    }
}
