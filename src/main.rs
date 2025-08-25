#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _periphs = embassy_rp::init(Default::default());
    let fw = include_bytes!("../cyw43/43439A0.bin");
    let clm = include_bytes!("../cyw43/43439A0_clm.bin");

    loop {
        defmt::info!("Blink");
        Timer::after(Duration::from_millis(100)).await;
    }
}
