mod command;
pub mod v1;
pub mod v2;

pub use command::ColorFormat;
use heapless::String;

#[derive(Debug)]
pub enum Error<SpiError, DcError, RstError> {
    Spi(SpiError),
    Dc(DcError),
    Reset(RstError),
    Coordinates(String<64>),
    DataBuffer(String<64>),
}
