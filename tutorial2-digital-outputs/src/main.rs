#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led_w = pins.d2.into_output();
    let mut led_y = pins.d3.into_output();
    let mut led_g = pins.d4.into_output();
    let mut led_r = pins.d5.into_output();

    loop {
        led_w.set_high();
        arduino_hal::delay_ms(500);
        led_w.set_low();
        arduino_hal::delay_ms(500);
        led_y.set_high();
        arduino_hal::delay_ms(500);
        led_y.set_low();
        arduino_hal::delay_ms(500);
        led_g.set_high();
        arduino_hal::delay_ms(500);
        led_g.set_low();
        arduino_hal::delay_ms(500);
        led_r.set_high();
        arduino_hal::delay_ms(500);
        led_r.set_low();
        arduino_hal::delay_ms(500);
    }
}
