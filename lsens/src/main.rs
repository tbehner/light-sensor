#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::port::Pin;
use arduino_hal::port::mode::Output;

struct LedWithThreshold {
    led: Pin<Output> ,
    lower_threshold: u16,
    upper_threshold: u16,
}

impl LedWithThreshold {
    fn new(mut led: Pin<Output>, lower_threshold: u16, upper_threshold: u16) -> LedWithThreshold {
        led.set_low();
        LedWithThreshold {led, lower_threshold, upper_threshold}
    }

    fn set(&mut self, value: u16) {
        if value < self.lower_threshold && self.led.is_set_low(){
            let _ = self.led.set_high();
        } else if value > self.upper_threshold && self.led.is_set_high() {
            let _ = self.led.set_low();
        }
    }
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let led = pins.d13.into_output().downgrade();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let sensor_input = pins.a0.into_analog_input(&mut adc);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut led_thr = LedWithThreshold::new(led, 300, 500);

    loop {
        arduino_hal::delay_ms(1000);
        let value = sensor_input.analog_read(&mut adc);

        led_thr.set(value);

        let _ = ufmt::uwriteln!(serial, "{}", value);
    }
}
