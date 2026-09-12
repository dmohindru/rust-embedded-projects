use crate::shift_register::SerialInput;
pub struct InputReport {
    menu_btn_pressed: bool,
    b_btn_pressed: bool,
    a_btn_pressed: bool,
    right_btn_pressed: bool,
    down_btn_pressed: bool,
    up_btn_pressed: bool,
    left_btn_pressed: bool,
}

struct MakeCodeInput<I: SerialInput> {
    serial_device: I,
}

impl<I: SerialInput> MakeCodeInput<I> {
    pub fn new(serial_device: I) -> Self {
        Self { serial_device }
    }

    pub async fn read_input_report(&mut self) -> Result<InputReport, I::Error> {
        let mut data = self.serial_device.read::<1>().await?[0];
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

    struct MockSerialInput {
        data: u8,
    }

    impl MockSerialInput {
        fn new(data: u8) -> Self {
            MockSerialInput { data }
        }
    }

    // impl SerialInput for MockSerialInput {

    // }

    #[tokio::test]
    async fn should_return_error_if_serial_input_device_returns_error() {
        todo!()
    }

    #[tokio::test]
    async fn should_report_buttons_pressed_on_appropriate_serial_input_data() {
        todo!()
    }

    #[tokio::test]
    async fn should_report_buttons_not_pressed_on_appropriate_serial_input_data() {
        todo!()
    }

    fn get_serial_input(data: u8) -> MockSerialInput {
        MockSerialInput::new(data)
    }
}
