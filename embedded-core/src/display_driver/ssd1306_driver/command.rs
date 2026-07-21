use crate::display_driver::Encode;
pub enum CommandMode {
    Control,
    Data,
}
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
    TopToBottom, // C0
    BottomToTop, // C8
}

pub enum SegmentRemap {
    Normal,   // A0
    Remapped, // A1
}

// See section 2.1 Command Table for Charge Bump Setting
pub enum PowerMode {
    InternalChargePump,
    ExternalVcc,
}

pub enum DisplaySize {
    Display128x64, // DA 12
    Display128x32, // DA 02
    Display96x16,  // DA 02
}
pub enum Command {
    ControlByte(CommandMode),
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
            Command::ControlByte(mode) => match mode {
                CommandMode::Control => {
                    out[0] = 0x00;
                    1
                }
                CommandMode::Data => {
                    out[0] = 0x40;
                    1
                }
            },
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
            Command::SetDisplayMode(mode) => match mode {
                DisplayMode::Inverted => {
                    out[0] = 0xA7;
                    1
                }
                DisplayMode::Normal => {
                    out[0] = 0xA6;
                    1
                }
            },
            Command::EnableDisplay(enable) => match enable {
                true => {
                    out[0] = 0xAF;
                    1
                }
                false => {
                    out[0] = 0xAE;
                    1
                }
            },
            Command::SetMemoryAddressMode(mode) => match mode {
                AddressMode::Horizontal => {
                    out[0] = 0x20;
                    out[1] = 0x00;
                    2
                }
                AddressMode::Vertical => {
                    out[0] = 0x20;
                    out[1] = 0x01;
                    2
                }
                AddressMode::Page => {
                    out[0] = 0x20;
                    out[1] = 0x02;
                    2
                }
            },
            Command::SetColumnAddress(addr) => {
                out[0] = 0x21;
                out[1] = 0x7F & addr[0];
                out[2] = 0x7F & addr[1];
                3
            }
            Command::SetPageAddress(addr) => {
                out[0] = 0x22;
                out[1] = 0x07 & addr[0];
                out[2] = 0x07 & addr[1];
                3
            }
            Command::SetDisplayStartLine(offset) => {
                out[0] = 0x40 | (0x3F & offset);
                1
            }
            Command::SetSegmentRemap(remap_mode) => match remap_mode {
                SegmentRemap::Normal => {
                    out[0] = 0xA0;
                    1
                }
                SegmentRemap::Remapped => {
                    out[0] = 0xA1;
                    1
                }
            },
            Command::SetMultiplexRatio(ratio) => {
                if *ratio > 0x00 && *ratio < 0x0F {
                    panic!("Invalid set multiplex ratio argument")
                } else {
                    out[0] = 0xA8;
                    out[1] = 0x3F & ratio;
                    2
                }
            }
            Command::SetScanDirection(direction) => match direction {
                ScanDirection::TopToBottom => {
                    out[0] = 0xC0;
                    1
                }
                ScanDirection::BottomToTop => {
                    out[0] = 0xC8;
                    1
                }
            },
            Command::SetDisplayOffset(offset) => {
                out[0] = 0xD3;
                out[1] = 0x3F & offset;
                2
            }
            Command::SetComPinConfig(config) => match config {
                DisplaySize::Display128x64 => {
                    out[0] = 0xDA;
                    out[1] = 0x12;
                    2
                }
                DisplaySize::Display128x32 => {
                    out[0] = 0xDA;
                    out[1] = 0x02;
                    2
                }
                DisplaySize::Display96x16 => {
                    out[0] = 0xDA;
                    out[1] = 0x02;
                    2
                }
            },
            Command::SetClockDivider(data) => {
                out[0] = 0xD5;
                out[1] = *data;
                2
            }
            Command::SetPreCharge(data) => {
                out[0] = 0xD9;
                out[1] = *data;
                2
            }
            Command::SetVComLevel => {
                out[0] = 0xDB;
                out[1] = 0x40;
                2
            }
            Command::SetChargePump(power_mode) => match power_mode {
                PowerMode::ExternalVcc => {
                    out[0] = 0x8D;
                    out[1] = 0x10;
                    2
                }
                PowerMode::InternalChargePump => {
                    out[0] = 0x8D;
                    out[1] = 0x14;
                    2
                }
            },
        };
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_provide_control_byte_encoding() {
        let command = Command::ControlByte(CommandMode::Control);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0x00, out[0]);

        let command = Command::ControlByte(CommandMode::Data);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0x40, out[0]);
    }

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
        assert_eq!(0xAF, out[0]);

        let command = Command::EnableDisplay(false);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xAE, out[0]);
    }

    #[test]
    fn should_provide_set_memory_address_encoding() {
        let command = Command::SetMemoryAddressMode(AddressMode::Horizontal);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0x20, out[0]);
        // Second byte
        assert_eq!(0x00, out[1]);

        let command = Command::SetMemoryAddressMode(AddressMode::Vertical);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0x20, out[0]);
        // Second byte
        assert_eq!(0x01, out[1]);

        let command = Command::SetMemoryAddressMode(AddressMode::Page);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0x20, out[0]);
        // Second byte
        assert_eq!(0x02, out[1]);
    }

    #[test]
    fn should_provide_set_column_address_encoding() {
        let command = Command::SetColumnAddress([0x00, 0xFF]);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(3, len);
        // First byte
        assert_eq!(0x21, out[0]);
        // Second byte
        assert_eq!(0x00, out[1]);
        // Third byte
        assert_eq!(0x7F, out[2]);
    }

    #[test]
    fn should_provide_set_page_address_encoding() {
        let command = Command::SetPageAddress([0x00, 0xFF]);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(3, len);
        // First byte
        assert_eq!(0x22, out[0]);
        // Second byte
        assert_eq!(0x00, out[1]);
        // Third byte
        assert_eq!(0x07, out[2]);
    }

    #[test]
    fn should_provide_set_display_start_line_encoding() {
        let command = Command::SetDisplayStartLine(0xFF);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0x7F, out[0]);

        let command = Command::SetDisplayStartLine(0x00);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0x40, out[0]);
    }

    #[test]
    fn should_provide_set_segment_remap_encoding() {
        let command = Command::SetSegmentRemap(SegmentRemap::Normal);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xA0, out[0]);

        let command = Command::SetSegmentRemap(SegmentRemap::Remapped);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xA1, out[0]);
    }

    #[test]
    #[should_panic(expected = "Invalid set multiplex ratio argument")]
    fn should_panic_for_invalid_set_multiplex_ration_encoding() {
        let command = Command::SetMultiplexRatio(0x00);
        let mut out: [u8; 4] = [0; 4];
        command.encode(&mut out);

        let command = Command::SetMultiplexRatio(0x0E);
        let mut out: [u8; 4] = [0; 4];
        command.encode(&mut out);
    }

    #[test]
    fn should_provide_set_multiplex_ratio_encoding() {
        let command = Command::SetMultiplexRatio(0x0F);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xA8, out[0]);
        // Second byte
        assert_eq!(0x0F, out[1]);

        let command = Command::SetMultiplexRatio(0xFF);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xA8, out[0]);
        // Second byte
        assert_eq!(0x3F, out[1]);
    }

    #[test]
    fn should_provide_set_scan_direction_encoding() {
        let command = Command::SetScanDirection(ScanDirection::TopToBottom);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xC0, out[0]);

        let command = Command::SetScanDirection(ScanDirection::BottomToTop);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(1, len);
        // First byte
        assert_eq!(0xC8, out[0]);
    }

    #[test]
    fn should_provide_set_display_offset_encoding() {
        let command = Command::SetDisplayOffset(0x00);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xD3, out[0]);
        // Second byte
        assert_eq!(0x00, out[1]);

        let command = Command::SetDisplayOffset(0xFF);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xD3, out[0]);
        // Second byte
        assert_eq!(0x3F, out[1]);
    }

    #[test]
    fn should_provide_set_com_pin_config_encoding() {
        let command = Command::SetComPinConfig(DisplaySize::Display128x64);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xDA, out[0]);
        // Second byte
        assert_eq!(0x12, out[1]);

        let command = Command::SetComPinConfig(DisplaySize::Display128x32);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xDA, out[0]);
        // Second byte
        assert_eq!(0x02, out[1]);

        let command = Command::SetComPinConfig(DisplaySize::Display96x16);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xDA, out[0]);
        // Second byte
        assert_eq!(0x02, out[1]);
    }

    #[test]
    fn should_provide_set_clock_divider_encoding() {
        let command = Command::SetClockDivider(0x00);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xD5, out[0]);
        // Second byte
        assert_eq!(0x00, out[1]);

        let command = Command::SetClockDivider(0xFF);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xD5, out[0]);
        // Second byte
        assert_eq!(0xFF, out[1]);
    }

    #[test]
    fn should_provide_set_pre_charge_encoding() {
        let command = Command::SetPreCharge(0x00);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xD9, out[0]);
        // Second byte
        assert_eq!(0x00, out[1]);

        let command = Command::SetPreCharge(0xFF);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xD9, out[0]);
        // Second byte
        assert_eq!(0xFF, out[1]);
    }

    #[test]
    fn should_provide_set_vcomh_level_encoding() {
        let command = Command::SetVComLevel;
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0xDB, out[0]);
        // Second byte
        assert_eq!(0x40, out[1]);
    }

    #[test]
    fn should_provide_set_charge_pump_encoding() {
        let command = Command::SetChargePump(PowerMode::ExternalVcc);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0x8D, out[0]);
        // Second byte
        assert_eq!(0x10, out[1]);

        let command = Command::SetChargePump(PowerMode::InternalChargePump);
        let mut out: [u8; 4] = [0; 4];
        let len = command.encode(&mut out);
        assert_eq!(2, len);
        // First byte
        assert_eq!(0x8D, out[0]);
        // Second byte
        assert_eq!(0x14, out[1]);
    }
}
