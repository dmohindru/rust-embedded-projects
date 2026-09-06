use embedded_hal::digital::{InputPin, OutputPin};
pub struct Hc165<I, O>
where
    I: InputPin,
    O: OutputPin,
{
    clk: O,
    clk_inhabit: Option<O>,
    data_in: I,
}

impl<I, O> Hc165<I, O>
where
    I: InputPin,
    O: OutputPin,
{
    pub fn new(clk: O, clk_inhabit: Option<O>, data_in: I) -> Self {
        Self {
            clk,
            clk_inhabit,
            data_in,
        }
    }
}
