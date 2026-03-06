use crate::frame::Frame;
use embedded_hal_async::spi::SpiDevice;

pub struct Max7219<D, const R: usize, const C: usize>
where
    D: SpiDevice,
{
    device: D,
}

impl<D, const R: usize, const C: usize> Max7219<D, R, C>
where
    D: SpiDevice,
{
    const ASSERT_DIMENSIONS: () = {
        assert!(R == 8);
        assert!(C <= 8);
    };
    pub fn new(device: D) -> Self {
        let _ = Self::ASSERT_DIMENSIONS;
        Max7219 { device }
    }

    async fn write_reg(&mut self, addr: u8, data: u8) -> Result<(), D::Error> {
        self.device.write(&[addr, data]).await
    }

    pub async fn initialize(&mut self) -> Result<(), D::Error> {
        self.write_reg(0x0F, 0x00).await?; // display test off
        self.write_reg(0x0C, 0x00).await?; // shutdown
        self.write_reg(0x09, 0x00).await?; // decode mode
        self.write_reg(0x0B, 0x07).await?; // scan limit
        self.write_reg(0x0A, 0x08).await?; // intensity
        self.write_reg(0x0C, 0x01).await?; // normal operation
        Ok(())
    }

    pub async fn write_bitmap(&mut self, frame: &Frame<R, C>) -> Result<(), D::Error> {
        for r in 0..R {
            let row = (*frame.get_row(r) & 0xFF) as u8;
            self.write_reg((r + 1) as u8, row).await?;
        }
        Ok(())
    }

    pub async fn clear(&mut self) -> Result<(), D::Error> {
        for r in 0..8 {
            self.write_reg((r + 1) as u8, 0).await?;
        }
        Ok(())
    }

    pub async fn set_intensity(&mut self, val: u8) -> Result<(), D::Error> {
        self.write_reg(0x0A, val & 0x0F).await
    }

    // TODO
    /*Future Feature (Advanced)
    Right now every update sends 8 SPI transactions.
    Better performance would send one transaction:
    [addr,data][addr,data][addr,data]...
    */
}

#[cfg(test)]
impl<D, const R: usize, const C: usize> Max7219<D, R, C>
where
    D: SpiDevice,
{
    pub fn free(self) -> D {
        self.device
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};

    #[tokio::test]
    async fn should_initialize_with_proper_commands() {
        let expectations: [SpiTransaction<u8>; 8] = [
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x0F, 0x00]),
            SpiTransaction::write_vec(vec![0x0C, 0x00]),
            SpiTransaction::write_vec(vec![0x09, 0x00]),
            SpiTransaction::write_vec(vec![0x0B, 0x07]),
            SpiTransaction::write_vec(vec![0x0A, 0x08]),
            SpiTransaction::write_vec(vec![0x0C, 0x01]),
            SpiTransaction::transaction_end(),
        ];

        let spi = SpiMock::new(&expectations);

        let mut max7219_device = Max7219::<_, 8, 8>::new(spi);
        max7219_device.initialize().await.unwrap();

        max7219_device.free().done();
    }
}
