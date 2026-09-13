use crate::shift_register::SerialInput;

pub struct InputReport {
    pub menu_btn_pressed: bool,
    pub b_btn_pressed: bool,
    pub a_btn_pressed: bool,
    pub right_btn_pressed: bool,
    pub down_btn_pressed: bool,
    pub up_btn_pressed: bool,
    pub left_btn_pressed: bool,
}

pub struct MakeCodeInput<I: SerialInput<1>> {
    serial_device: I,
}

impl<I: SerialInput<1>> MakeCodeInput<I> {
    pub fn new(serial_device: I) -> Self {
        Self { serial_device }
    }

    pub async fn read_input_report(&mut self) -> Result<InputReport, I::Error> {
        let mut data = self.serial_device.read().await?[0];
        let left_btn_pressed = data & 0x01 == 0;
        data = data >> 1;
        let up_btn_pressed = data & 0x01 == 0;
        data = data >> 1;
        let down_btn_pressed = data & 0x01 == 0;
        data = data >> 1;
        let right_btn_pressed = data & 0x01 == 0;
        data = data >> 1;
        let a_btn_pressed = data & 0x01 == 0;
        data = data >> 1;
        let b_btn_pressed = data & 0x01 == 0;
        data = data >> 1;
        let menu_btn_pressed = data & 0x01 == 0;
        Ok(InputReport {
            menu_btn_pressed,
            b_btn_pressed,
            a_btn_pressed,
            right_btn_pressed,
            down_btn_pressed,
            up_btn_pressed,
            left_btn_pressed,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io;

    struct MockSerialInput<const N: usize> {
        data: [u8; N],
        is_error: bool,
    }

    impl<const N: usize> MockSerialInput<N> {
        fn new(data: [u8; N], is_error: bool) -> Self {
            MockSerialInput { data, is_error }
        }
    }

    impl<const N: usize> SerialInput<N> for MockSerialInput<N> {
        type Error = io::ErrorKind;

        async fn read(&mut self) -> Result<[u8; N], Self::Error> {
            if self.is_error {
                Err(io::ErrorKind::Other)
            } else {
                Ok(self.data)
            }
        }
    }

    #[tokio::test]
    async fn should_return_error_if_serial_input_device_returns_error() {
        let mock_input = get_serial_input(0u8, true);
        let mut make_code_input = MakeCodeInput::new(mock_input);
        let result = make_code_input.read_input_report().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_report_buttons_pressed_on_appropriate_serial_input_data() {
        let mock_input = get_serial_input(0u8, false);
        let mut make_code_input = MakeCodeInput::new(mock_input);
        let result = make_code_input.read_input_report().await.unwrap();
        assert!(result.a_btn_pressed);
        assert!(result.b_btn_pressed);
        assert!(result.right_btn_pressed);
        assert!(result.down_btn_pressed);
        assert!(result.up_btn_pressed);
        assert!(result.left_btn_pressed);
        assert!(result.menu_btn_pressed);
    }

    #[tokio::test]
    async fn should_report_buttons_not_pressed_on_appropriate_serial_input_data() {
        let mock_input = get_serial_input(0x7F, false);
        let mut make_code_input = MakeCodeInput::new(mock_input);
        let result = make_code_input.read_input_report().await.unwrap();
        assert!(!result.a_btn_pressed);
        assert!(!result.b_btn_pressed);
        assert!(!result.right_btn_pressed);
        assert!(!result.down_btn_pressed);
        assert!(!result.up_btn_pressed);
        assert!(!result.left_btn_pressed);
        assert!(!result.menu_btn_pressed);
    }

    fn get_serial_input(data_bits: u8, is_error: bool) -> MockSerialInput<1> {
        MockSerialInput::new([data_bits], is_error)
    }
}
