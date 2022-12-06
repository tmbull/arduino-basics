#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut delay_val = 200;

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let button = pins.d12.into_pull_up_input();

    let led_w = pins.d2.into_output().downgrade();
    let led_y = pins.d3.into_output().downgrade();
    let led_g = pins.d4.into_output().downgrade();
    let led_r = pins.d5.into_output().downgrade();

    let mut leds: [Pin<Output, Dynamic>; 4] = [led_w, led_y, led_g, led_r];
    let mut current_led = 0;

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    loop {
        ufmt::uwriteln!(&mut serial, "Button state: '{}', current led: {}", button.is_low(), current_led);
        if button.is_low() {
            if current_led == 2 {
                leds[2].set_high();
                delay_ms(delay_val);
                leds[2].set_low();
                delay_ms(delay_val);
                leds[2].set_high();
                delay_ms(delay_val);
                leds[2].set_low();
                delay_ms(delay_val);

                delay_val -= 20;
            } else {
                leds[current_led].set_high();
                delay_ms(delay_val);
                leds[current_led].set_low();
                delay_ms(delay_val);
                leds[current_led].set_high();
                delay_ms(delay_val);
                leds[current_led].set_low();
                delay_ms(delay_val);
            }
        }

        leds[current_led].set_high();
        delay_ms(delay_val);
        leds[current_led].set_low();
        delay_ms(delay_val);
        current_led = (current_led + 1) % 4;
    }
}
