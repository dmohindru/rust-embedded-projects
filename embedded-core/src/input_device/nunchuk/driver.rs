use embedded_hal_async::i2c::I2c;

pub struct Nunchuk<D>
where
    D: I2c,
{
    device: D,
    address: u8,
}

impl<D> Nunchuk<D>
where
    D: I2c,
{
    pub fn new(device: D, address: u8) -> Self {
        Self { device, address }
    }
}
