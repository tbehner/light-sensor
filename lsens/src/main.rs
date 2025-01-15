#![no_std]
#![no_main]

use embedded_hal::digital::{v2::OutputPin, v2::StatefulOutputPin};
use panic_halt as _;
use arduino_hal::port::Pin;
use arduino_hal::port::mode::Output;

struct LedWithThreshold {
    led: Pin<Output> ,
    lower_threshold: u16,
    upper_threshold: u16,
}

impl LedWithThreshold {
    fn new(led: Pin<Output>, lower_threshold: u16, upper_threshold: u16) -> LedWithThreshold {
        LedWithThreshold {led, lower_threshold, upper_threshold}
    }

    fn set(&mut self, value: u16) {
        if value > 1000 && self.led.is_set_low(){
            let _ = self.led.set_low();
        } else if value < 800 && self.led.is_set_high() {
            let _ = self.led.set_high();
        }
    }
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output().downgrade();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let sensor_input = pins.a0.into_analog_input(&mut adc);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut led_thr = LedWithThreshold::new(led, 800, 1000);

    loop {
        arduino_hal::delay_ms(1000);
        let value = sensor_input.analog_read(&mut adc);

        led_thr.set(value);

        let _ = ufmt::uwriteln!(serial, "{}", voltage);
    }
}
