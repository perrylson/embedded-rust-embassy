#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::Config;
use embassy_time::{Duration, Timer};
mod sensor;
use crate::sensor::sensor::{DPS310, OversampleRate};
use {defmt_rtt as _, panic_probe as _};


bind_interrupts!(struct Irqs {
        I2C1_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C1>;
});


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_14;
    let scl = p.PIN_15;
    let mut i2c = embassy_rp::i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());

    Timer::after(Duration::from_millis(100)).await;

    let mut dps310 = DPS310::new(&mut i2c).await;
    dps310
        .set_temperature_oversampling_rate(OversampleRate::Sixteen)
        .await;
    dps310
        .set_pressure_oversampling_rate(OversampleRate::Sixteen)
        .await;

    dps310.read_temperature().await;
    dps310.read_pressure().await;
}
