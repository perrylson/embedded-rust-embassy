# rust-dps310-driver
Tested the DPS310 temperature and pressure sensor with the Raspberry Pi Pico 2 W. Used the Embassy library to set up an I2C driver interface. Connected SDA and SCL to pin 14 and 15, respectively. Currently supports "command" mode.
![Raspberry Pi Pico 2 W](images/pico-2.png)

### Get started
- Run `cargo run` to see the sensor's temperature and pressure data

### Example output
If embedded program was successfully ran, example output should look something like this:
```
0.106701 [INFO ] Checking temperature read status... (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:173)
0.207168 [INFO ] Checking temperature read status... (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:173)
0.208233 [INFO ] Celsius: 22.649059664818537  Fahrenheit: 72.76830739667336 (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:245)
0.208699 [INFO ] Checking temperature read status... (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:173)
0.309148 [INFO ] Checking temperature read status... (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:173)
0.310543 [INFO ] Checking pressure read status... (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:199)
0.410995 [INFO ] Checking pressure read status... (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:199)
0.412056 [INFO ] Pressure (Pa): 101647.20279845835 (rust_dps310_driver embedded-rust-embassy/src/sensor.rs:302)
```

Note: You'll need to connect a Raspberry Pi debugging probe to the microcontroller in order to see the data logs.  
