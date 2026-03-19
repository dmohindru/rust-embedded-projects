#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::spi::{Config, Spi};
use embassy_time::{Delay, Timer};
use embedded_core::button::DebouncedButton;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn button_task(
    mut button: DebouncedButton<Input<'static>, Delay>,
    mut spi: Spi<'static, embassy_rp::peripherals::SPI1, embassy_rp::spi::Async>,
) {
    loop {
        // button
        //     .wait(|| async {
        //         info!("Button pressed");
        //     })
        //     .await;
        let tx_buf = [1_u8, 2, 3, 4, 5, 6];
        let mut rx_buf = [0_u8; 6];
        spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
        info!("{:?}", rx_buf);
        Timer::after_secs(2).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let miso = p.PIN_12;
    let mosi = p.PIN_11;
    let clk = p.PIN_10;

    let btn_a = Input::new(p.PIN_15, Pull::Up);
    let debounced_btn_a = DebouncedButton::new(btn_a, Delay, 20);

    let spi = Spi::new(
        p.SPI1,
        clk,
        mosi,
        miso,
        p.DMA_CH0,
        p.DMA_CH1,
        Config::default(),
    );

    spawner
        .spawn(button_task(debounced_btn_a, spi))
        .expect("Failed to spawn button a task");
}
