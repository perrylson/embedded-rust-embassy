# embedded-rust-embassy
Tested the DPS310 temperature and pressure sensor with the Raspberry Pi Pico 2 W. Used the Embassy library to set up a driver interface. Connected SDA and SCL to pin 14 and 15, respectively. 

### Get started
- Run `cargo run` to see the sensor's temperature and pressure data

### Example output
If embedded program was successfully ran, example output should look something like this:
```
0.106742 [INFO ] Checking temperature read status... (pico embedded-rust-embassy/src/sensor.rs:179)
0.207205 [INFO ] Checking temperature read status... (pico embedded-rust-embassy/src/sensor.rs:179)
0.208270 [INFO ] Celsius: 29.032254126764116  Fahrenheit: 84.2580574281754 (pico embedded-rust-embassy/src/sensor.rs:251)
0.208743 [INFO ] Checking temperature read status... (pico embedded-rust-embassy/src/sensor.rs:179)
0.309190 [INFO ] Checking temperature read status... (pico embedded-rust-embassy/src/sensor.rs:179)
0.310581 [INFO ] Checking pressure read status... (pico embedded-rust-embassy/src/sensor.rs:205)
0.411032 [INFO ] Checking pressure read status... (pico embedded-rust-embassy/src/sensor.rs:205)
0.412092 [INFO ] Pressure (Pa): 100909.12957427498 (pico embedded-rust-embassy/src/sensor.rs:308)
```

Note: You'll need to connect a Raspberry Pi debugging probe to the microcontroller in order to see the data logs.  
