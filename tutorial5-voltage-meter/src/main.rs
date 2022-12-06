#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::adc;
use arduino_hal::prelude::*;
use ufmt::{uwriteln};
use ufmt_float::uFmt_f32;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).void_unwrap();

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let analog_in = pins.a0.into_analog_input(&mut adc);

    loop {
        let analog_val = analog_in.analog_read(&mut adc) as f32;
        let voltage = analog_val as f32 * (5.0 / 1023.0);
        let voltage_write = uFmt_f32::Five(voltage);
        ufmt::uwriteln!(&mut serial, "Voltage is '{}'", voltage_write).void_unwrap();

        arduino_hal::delay_ms(1000);
    }
}
