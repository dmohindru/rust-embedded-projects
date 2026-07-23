pub trait Present {
    type Error;

    fn present(&mut self) -> Result<(), Self::Error>;
}
