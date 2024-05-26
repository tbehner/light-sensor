#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut _led = pins.d13.into_output();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let sensor_input = pins.a0.into_analog_input(&mut adc);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        arduino_hal::delay_ms(100);
	let voltage = sensor_input.analog_read(&mut adc);
	_ = ufmt::uwriteln!(serial, "{}\n", voltage);
    }
}
