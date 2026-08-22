use crate::input::input_description::InputId;

pub enum InputValue {
    Button(bool),
    Axis(i16),
    Acceleration(i16),
}
pub struct InputState {}

impl InputState {
    pub fn get(&self, _id: InputId) -> Option<InputValue> {
        todo!()
    }
}
