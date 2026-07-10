use crate::display_driver::Encode;
pub enum DisplayMode {
    Normal,
    Inverted,
}
pub enum AddressMode {
    Horizontal,
    Vertical,
    Page,
}

pub enum ScanDirection {
    TopToBottom,
    BottomToTop,
}

pub enum SegmentRemap {
    Normal,   // A0
    Remapped, // A1
}

pub enum ComScanDirection {
    Normal,   // C0
    Remapped, // C8
}

// See section 2.1 Command Table for Charge Bump Setting
enum PowerMode {
    InternalChargePump,
    ExternalVcc,
}

pub enum DisplaySize {
    Display128x64, // DA 12
    Display128x32, // DA 02
    Display96x16,  // DA 02
}
pub enum Command {
    /// Fundamental Commands
    SetContrast(u8),
    SetDisplayMode(DisplayMode),
    EnableDisplay(bool),
    EnableRamContent(bool),
    /// Address settings commands
    SetColumnAddress([u8; 2]),
    SetPageAddress([u8; 2]),
    SetMemoryAddressMode(AddressMode),
    /// Hardware config commands
    SetDisplayStartLine(u8),
    SetSegmentRemap(SegmentRemap),
    SetMultiplexRatio(u8),
    SetScanDirection(ScanDirection),
    SetDisplayOffset(u8),
    SetComPinConfig(DisplaySize),
    /// Timing & Driving scheme setting Command
    /// Done expose any of the timing commands
    SetClockDivider(u8),
    SetPreCharge(u8),
    SetVComLevel,
    /// Electrical commands
    SetChargePump(PowerMode),
}

impl Encode for Command {
    fn encode(&self, out: &mut [u8]) -> usize {
        let length: usize = match self {
            Command::SetContrast(level) => {
                out[0] = 0x81;
                out[1] = *level;
                2
            }
            Command::EnableRamContent(enable) => match enable {
                true => {
                    out[0] = 0xA4;
                    1
                }
                false => {
                    out[0] = 0xA5;
                    1
                }
            },
            _ => todo!(),
        };
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // static mut out: [u8; 4] = [0; 4];
    #[test]
    fn should_provide_set_contrast_encoding() {
        let command = Command::SetContrast(0xF0);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0x81, out[0]);
        // Second byte
        assert_eq!(0xF0, out[1]);
    }

    #[test]
    fn should_provide_enable_ram_content_encoding() {
        let command = Command::EnableRamContent(true);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xA4, out[0]);

        let command = Command::EnableRamContent(false);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xA5, out[0]);
    }

    #[test]
    fn should_provide_set_display_mode_encoding() {
        let command = Command::SetDisplayMode(DisplayMode::Normal);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xA6, out[0]);

        let command = Command::SetDisplayMode(DisplayMode::Inverted);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xA7, out[0]);
    }

    #[test]
    fn should_provide_enable_display_encoding() {
        let command = Command::EnableDisplay(true);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xAE, out[0]);

        let command = Command::EnableDisplay(false);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xAF, out[0]);
    }
}

// PowerMode::InternalChargePump => 0x14,
// PowerMode::ExternalVcc         => 0x10,

// Note: The SSD1306 datasheet's bit-field description for the VCOMH
// command is misleading. The values 0x00, 0x40, and 0x60 are the
// ones used by the datasheet's examples and by all major drivers.

/*
D5 80       Clock divide -- done --
understand its arguments.
Set the Oscillator Frequency, FOSC.
Oscillator Frequency increases with
the value of A[7:4] and vice versa.
RESET is 1000b

D3 00       Display offset = 0 -- done --
Set vertical shift by COM from 0d~63d
The value is reset to 00h after RESET.

40          Start line = 0 -- done --
Set display RAM display start line register from
0-63 using X5X3X2X1X0.
Display start line register is reset to 000000b
during RESET.

8D 14       Charge pump ON -- done --
Cant find documentation for this command
See section 2.1 Command Table for Charge Bump Setting

A1          Segment remap -- done --
A0h, X[0]=0b: column address 0 is mapped to
SEG0 (RESET)

C8          COM scan direction remapped -- done --
C8h, X[3]=1b: remapped mode. Scan from
COM[N-1] to COM0
Why not C0 ?

DA 12       COM pins configuration -- done --
What exactly will this command really do?
I find reset value is
A[4]=0b, Sequential COM pin configuration
A[4]=1b(RESET), Alternative COM pin
configuration
A[5]=0b(RESET), Disable COM Left/Right
remap
A[5]=1b, Enable COM Left/Right remap

81 CF       Contrast
(RESET = 7Fh ) is then why CF?

D9 F1       Pre-charge
What would this command really do?

DB 40       VCOMH level
What does this command do?
I believe argument 40 doesn't seems valid
As per docs
A[6:0] allowed values are 000, 010, 011

A6          Normal display
It has this value on reset any ways
A6h, X[0]=0b: Normal display (RESET)
*/
