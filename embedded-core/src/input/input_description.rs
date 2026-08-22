pub enum InputKind {
    Button,
    Axis { min: i16, max: i16 },
    Acceleration,
}

#[derive(Copy, Clone)]
pub struct InputId(pub u8);

pub struct InputDescription {
    pub id: InputId,
    pub label: &'static str,
    pub kind: InputKind,
    pub unit: Option<&'static str>,
}

pub struct InputDescriptor {
    pub name: &'static str,
    pub inputs: &'static [InputDescription],
}
