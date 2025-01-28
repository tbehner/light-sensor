#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    led.set_low();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let sensor_input = pins.a0.into_analog_input(&mut adc);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        arduino_hal::delay_ms(1000);
        let voltage = sensor_input.analog_read(&mut adc);

        if voltage < 400 && led.is_set_low(){
            led.set_high();
        } else if voltage > 500 && led.is_set_high() {
            led.set_low();
        }

        let _ = ufmt::uwriteln!(serial, "{}", voltage);
    }
}
