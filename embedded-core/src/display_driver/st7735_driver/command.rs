use crate::display_driver::Encode;

pub enum ColorFormat {
    TwelveBPP,
    SixteenBPP,
    EighteenBPP,
}

pub enum Command {
    SWRESET,
    SLPOUT,
    FRMCTR1([u8; 3]),
    FRMCTR2([u8; 3]),
    FRMCTR3([u8; 6]),
    INVCTR(u8),
    PWCTR1([u8; 3]),
    PWCTR2(u8),
    PWCTR3([u8; 2]),
    PWCTR4([u8; 2]),
    PWCTR5([u8; 2]),
    VMCTR1(u8),
    MADCTL(u8),
    COLMOD(ColorFormat),
    GAMCTRP1([u8; 16]),
    GAMCTRN1([u8; 16]),
    DISPON,
    NORON,
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
                out[3] = (parameters[2] << 6) | 0x04;
                4
            }
            Command::PWCTR2(parameter) => {
                out[0] = 0xC1;
                out[1] = *parameter & 0xCF;
                2
            }
            Command::PWCTR3(parameters) => {
                out[0] = 0xC2;
                out[1] = parameters[0];
                out[2] = parameters[1];
                3
            }
            Command::PWCTR4(parameters) => {
                out[0] = 0xC3;
                out[1] = parameters[0];
                out[2] = parameters[1];
                3
            }
            Command::PWCTR5(parameters) => {
                out[0] = 0xC4;
                out[1] = parameters[0];
                out[2] = parameters[1];
                3
            }
            Command::VMCTR1(parameter) => {
                out[0] = 0xC5;
                out[1] = *parameter & 0x3F;
                2
            }
            Command::MADCTL(parameter) => {
                out[0] = 0x36;
                out[1] = *parameter & 0xFC;
                2
            }
            Command::COLMOD(format) => {
                out[0] = 0x3A;
                match format {
                    ColorFormat::TwelveBPP => out[1] = 0x03,
                    ColorFormat::SixteenBPP => out[1] = 0x05,
                    ColorFormat::EighteenBPP => out[1] = 0x06,
                }
                2
            }
            Command::GAMCTRP1(parameters) => {
                out[0] = 0xE0;
                for i in 0..16 {
                    out[i + 1] = parameters[i] & 0x3F;
                }
                17
            }
            Command::GAMCTRN1(parameters) => {
                out[0] = 0xE1;
                for i in 0..16 {
                    out[i + 1] = parameters[i] & 0x3F;
                }
                17
            }
            Command::DISPON => {
                out[0] = 0x29;
                1
            }
            Command::NORON => {
                out[0] = 0x13;
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

    #[test]
    fn should_provide_encoding_for_pwctr2_command() {
        let command = Command::PWCTR2(0xFF);
        let mut out: [u8; 2] = [0; 2];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        assert_eq!(0xC1, out[0]);
        assert_eq!(0xCF, out[1]);
    }

    #[test]
    fn should_provide_encoding_for_pwctr3_command() {
        let command = Command::PWCTR3([0xFF, 0xFF]);
        let mut out: [u8; 3] = [0; 3];
        let len = command.encode(&mut out);
        assert_eq!(3, len);
        assert_eq!(0xC2, out[0]);
        assert_eq!(0xFF, out[1]);
        assert_eq!(0xFF, out[2]);
    }

    #[test]
    fn should_provide_encoding_for_pwctr4_command() {
        let command = Command::PWCTR4([0xFF, 0xFF]);
        let mut out: [u8; 3] = [0; 3];
        let len = command.encode(&mut out);
        assert_eq!(3, len);
        assert_eq!(0xC3, out[0]);
        assert_eq!(0xFF, out[1]);
        assert_eq!(0xFF, out[2]);
    }

    #[test]
    fn should_provide_encoding_for_pwctr5_command() {
        let command = Command::PWCTR5([0xFF, 0xFF]);
        let mut out: [u8; 3] = [0; 3];
        let len = command.encode(&mut out);
        assert_eq!(3, len);
        assert_eq!(0xC4, out[0]);
        assert_eq!(0xFF, out[1]);
        assert_eq!(0xFF, out[2]);
    }

    #[test]
    fn should_provide_encoding_for_vmctr1_command() {
        let command = Command::VMCTR1(0xFF);
        let mut out: [u8; 2] = [0; 2];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        assert_eq!(0xC5, out[0]);
        assert_eq!(0x3F, out[1]);
    }

    #[test]
    fn should_provide_encoding_for_madctl_command() {
        let command = Command::MADCTL(0xFF);
        let mut out: [u8; 2] = [0; 2];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        assert_eq!(0x36, out[0]);
        assert_eq!(0xFC, out[1]);
    }

    #[test]
    fn should_provide_encoding_for_colmod_command() {
        // 12 bits per pixel
        let command = Command::COLMOD(ColorFormat::TwelveBPP);
        let mut out: [u8; 2] = [0; 2];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        assert_eq!(0x3A, out[0]);
        assert_eq!(0x03, out[1]);

        // 16 bits per pixel
        let command = Command::COLMOD(ColorFormat::SixteenBPP);
        let mut out: [u8; 2] = [0; 2];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        assert_eq!(0x3A, out[0]);
        assert_eq!(0x05, out[1]);

        // 18 bits per pixel
        let command = Command::COLMOD(ColorFormat::EighteenBPP);
        let mut out: [u8; 2] = [0; 2];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        assert_eq!(0x3A, out[0]);
        assert_eq!(0x06, out[1]);
    }

    #[test]
    fn should_provide_encoding_for_gamctrp1_command() {
        let command = Command::GAMCTRP1([
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ]);
        let mut out: [u8; 17] = [0; 17];
        let len = command.encode(&mut out);
        assert_eq!(17, len);
        assert_eq!(0xE0, out[0]);
        for i in 1..17 {
            assert_eq!(0x3F, out[i])
        }
    }

    #[test]
    fn should_provide_encoding_for_gamctrn1_command() {
        let command = Command::GAMCTRN1([
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ]);
        let mut out: [u8; 17] = [0; 17];
        let len = command.encode(&mut out);
        assert_eq!(17, len);
        assert_eq!(0xE1, out[0]);
        for i in 1..17 {
            assert_eq!(0x3F, out[i])
        }
    }

    #[test]
    fn should_provide_encoding_for_dispon_command() {
        let command = Command::DISPON;
        let mut out: [u8; 1] = [0; 1];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        assert_eq!(0x29, out[0]);
    }

    #[test]
    fn should_provide_encoding_for_noron_command() {
        let command = Command::NORON;
        let mut out: [u8; 1] = [0; 1];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        assert_eq!(0x13, out[0]);
    }
}
