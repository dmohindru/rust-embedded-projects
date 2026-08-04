pub trait Present {
    type Error;

    async fn present(&mut self) -> Result<(), Self::Error>;
}
