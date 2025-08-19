pub mod sensor {
    use embassy_rp::{i2c::Async, peripherals::I2C1};
    const SENSOR_ADDR: u8 = 0x77;
    const PROD_ID_REG_ADDR: u8 = 0x0D;
    const COEFFS_ADDR: u8 = 0x10;
    const PRESSURE_CONFIG_ADDR: u8 = 0x06;
    const TEMP_CONFIG_ADDR: u8 = 0x07;
    const MEAS_CFG_ADDR: u8 = 0x08;
    const CFG_REG_ADDR: u8 = 0x09;
    const TEMP_ADDR: u8 = 0x03;
    const PRESSURE_ADDR: u8 = 0x00;

    const SCALE_FACTORS: [i32; 8] = [
        524288, 1572864, 3670016, 7864320, 253952, 516096, 1040384, 2088960,
    ];

    fn get_twos_complement(value: u32, bit_length: i32) -> i32 {
        let casted_value = value as i32;
        if casted_value > ((1 << (bit_length - 1)) - 1) {
            casted_value - (1 << bit_length)
        } else {
            casted_value
        }
    }

    pub struct DPS310<'a> {
        i2c: &'a mut embassy_rp::i2c::I2c<'a, I2C1, Async>,
        coeffs: [i32; 9],
    }
    impl<'a> DPS310<'a> {
        pub async fn new(i2c: &'a mut embassy_rp::i2c::I2c<'a, I2C1, Async>) -> DPS310<'a> {
            let mut dps310_sensor = DPS310 {
                i2c,
                coeffs: [0; 9],
            };
            dps310_sensor
        }
    }
}
