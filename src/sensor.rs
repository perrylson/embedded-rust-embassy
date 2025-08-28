#![allow(dead_code)]
pub mod sensor {
    use defmt::info;
    use embassy_rp::{i2c::Async, peripherals::I2C1};
    use embassy_time::{Duration, Timer};
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
    pub enum OversampleRate {
        One,
        Two,
        Four,
        Eight,
        Sixteen,
        ThirtyTwo,
        SixtyFour,
        OneHundredTwentyEight,
    }

    pub struct DPS310<'a> {
        i2c: &'a mut embassy_rp::i2c::I2c<'a, I2C1, Async>,
        c0: i32,
        c1: i32,
        c00: i32,
        c10: i32,
        c01: i32,
        c11: i32,
        c20: i32,
        c21: i32,
        c30: i32,
        selected_temperature_scale_factor: i32,
        selected_pressure_scale_factor: i32,
    }
    impl<'a> DPS310<'a> {
        pub async fn new(i2c: &'a mut embassy_rp::i2c::I2c<'a, I2C1, Async>) -> DPS310<'a> {
            let mut dps310_sensor = DPS310 {
                i2c,
                c0: 0,
                c1: 0,
                c00: 0,
                c10: 0,
                c01: 0,
                c11: 0,
                c20: 0,
                c21: 0,
                c30: 0,
                selected_temperature_scale_factor: 0,
                selected_pressure_scale_factor: 0,
            };
            dps310_sensor.initialize_sensor().await;
            dps310_sensor
                .set_temperature_oversampling_rate(OversampleRate::Four)
                .await;
            dps310_sensor
                .set_pressure_oversampling_rate(OversampleRate::Four)
                .await;
            dps310_sensor
        }

        pub async fn reset_sensor(&mut self) {
            self.i2c
                .write_async(SENSOR_ADDR, [0x0C as u8, 0x9 as u8])
                .await
                .unwrap();
        }

        pub async fn set_temperature_oversampling_rate(&mut self, rate: OversampleRate) {
            let (reg_value, scale_factor) = match rate {
                OversampleRate::One => (0x80, SCALE_FACTORS[0]),
                OversampleRate::Two => (0x81, SCALE_FACTORS[1]),
                OversampleRate::Four => (0x82, SCALE_FACTORS[2]),
                OversampleRate::Eight => (0x83, SCALE_FACTORS[3]),
                OversampleRate::Sixteen => todo!(),
                OversampleRate::ThirtyTwo => todo!(),
                OversampleRate::SixtyFour => todo!(),
                OversampleRate::OneHundredTwentyEight => todo!(),
            };
            self.i2c
                .write_async(SENSOR_ADDR, [TEMP_CONFIG_ADDR, reg_value])
                .await
                .unwrap();
            self.selected_temperature_scale_factor = scale_factor;
        }

        pub async fn set_pressure_oversampling_rate(&mut self, rate: OversampleRate) {
            let (reg_value, scale_factor) = match rate {
                OversampleRate::One => (0x00, SCALE_FACTORS[0]),
                OversampleRate::Two => (0x01, SCALE_FACTORS[1]),
                OversampleRate::Four => (0x02, SCALE_FACTORS[2]),
                OversampleRate::Eight => (0x03, SCALE_FACTORS[3]),
                OversampleRate::Sixteen => todo!(),
                OversampleRate::ThirtyTwo => todo!(),
                OversampleRate::SixtyFour => todo!(),
                OversampleRate::OneHundredTwentyEight => todo!(),
            };
            self.i2c
                .write_async(SENSOR_ADDR, [PRESSURE_CONFIG_ADDR, reg_value])
                .await
                .unwrap();
            self.selected_pressure_scale_factor = scale_factor;
        }

        pub async fn initialize_sensor(&mut self) {
            let res = self.read_product_id().await;

            let mut temp_coef: [u8; 1] = [0];
            self.i2c
                .write_read_async(SENSOR_ADDR, [0x28 as u8], &mut temp_coef)
                .await
                .unwrap();

            self.initialize_coeffs().await;

            self.i2c
                .write_async(SENSOR_ADDR, [PRESSURE_CONFIG_ADDR, 0x02 as u8])
                .await
                .unwrap();
            self.i2c
                .write_async(SENSOR_ADDR, [TEMP_CONFIG_ADDR, 0x82 as u8])
                .await
                .unwrap();
            self.i2c
                .write_async(SENSOR_ADDR, [CFG_REG_ADDR, 0])
                .await
                .unwrap();
        }

        async fn check_temperature_status(&mut self) -> Result<&'static str, &'static str> {
            let mut meas_data: [u8; 1] = [0];
            let mut count = 0;
            let mut is_ready = false;
            while !is_ready && count < 5 {
                self.i2c
                    .write_read_async(SENSOR_ADDR, [MEAS_CFG_ADDR], &mut meas_data)
                    .await
                    .unwrap();

                if meas_data[0] & (1 << 5) != 0 {
                    info!("Temperature is ready to be read");
                    is_ready = true;
                } else {
                    info!("Temperature is not ready to be read");
                    Timer::after(Duration::from_millis(100)).await;
                }

                count += 1;
            }

            if is_ready {
                Ok("Successfully read temperature")
            } else {
                Err("Could not read temperature")
            }
        }

        pub async fn read_temperature(&mut self) {
            self.i2c
                .write_async(SENSOR_ADDR, [MEAS_CFG_ADDR, 0x02 as u8])
                .await
                .unwrap();

            self.check_temperature_status().await.unwrap();

            let mut temperature_data: [u8; 3] = [0; 3];
            self.i2c
                .write_read_async(SENSOR_ADDR, [TEMP_ADDR], &mut temperature_data)
                .await
                .unwrap();
            let raw_temp = get_twos_complement(
                ((temperature_data[0] as u32) << 16)
                    + ((temperature_data[1] as u32) << 8)
                    + (temperature_data[2] as u32),
                24,
            );
            info!("raw_temp: {} ", raw_temp);
            let proc_temp = (self.c0 as f64) * 0.5
                + (self.c1 as f64)
                    * ((raw_temp as f64) / (self.selected_temperature_scale_factor as f64));
            let temp_fahrenheit = (proc_temp * 9.0 / 5.0) + 32.0;
            info!("Celsius: {}  Fahrenheit: {}", proc_temp, temp_fahrenheit);
        }

        pub async fn read_product_id(&mut self) -> Result<&'static str, &'static str> {
            let mut result = [0];
            self.i2c
                .write_read_async(SENSOR_ADDR, [PROD_ID_REG_ADDR], &mut result)
                .await
                .unwrap();
            if result[0] == 16 {
                Ok("Was able to read product id")
            } else {
                Err("Could not find product id")
            }
        }

        pub async fn initialize_coeffs(&mut self) {
            let mut coeffs_raw: [u8; 18] = [0; 18];

            self.i2c
                .write_read_async(SENSOR_ADDR, [COEFFS_ADDR], &mut coeffs_raw)
                .await
                .unwrap();

            let result: [u32; 18] = coeffs_raw.map(|x| x as u32);

            self.c0 = get_twos_complement(((result[0]) << 4) + (((result[1]) >> 4) & 0x0F), 12);

            self.c1 = get_twos_complement((((result[1]) & 0x0F) << 8) + (result[2]), 12);

            self.c00 = get_twos_complement(
                ((result[3]) << 12) + ((result[4]) << 4) + (((result[5]) >> 4) & 0x0F),
                20,
            );

            self.c10 =
                get_twos_complement(((result[5]) << 16) + ((result[6]) << 8) + (result[7]), 20);

            self.c01 = get_twos_complement(((result[8]) << 8) + (result[9]), 16);

            self.c11 = get_twos_complement(((result[10]) << 8) + (result[11]), 16);

            self.c20 = get_twos_complement(((result[12]) << 8) + (result[13]), 16);

            self.c21 = get_twos_complement(((result[14]) << 8) + (result[15]), 16);

            self.c30 = get_twos_complement(((result[16]) << 8) + (result[17]), 16);
        }
    }
    
}
