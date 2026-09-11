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
        let data = self.serial_device.read::<1>().await?;
        todo!()
    }
}
