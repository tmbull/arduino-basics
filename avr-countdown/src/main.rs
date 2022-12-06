#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use embedded_hal::digital::v2::{OutputPin, PinState};
use embedded_hal::digital::v2::PinState::{High, Low};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let seven_seg_digits: [[PinState; 7]; 10] = [
        [Low,  Low,  Low,  Low,  Low,  Low,  High], // 0
        [High, Low,  Low,  High, High, High, High], // 1
        [Low,  Low,  High, Low,  Low,  High, Low ], // 2
        [Low,  Low,  Low,  Low,  High, High, Low ], // 3
        [High, Low,  Low,  High, High, Low,  Low ], // 4
        [Low,  High, Low,  Low,  High, Low,  Low ], // 5
        [Low,  High, Low,  Low,  Low,  Low,  Low ], // 6
        [Low,  Low,  Low,  High, High, High, High], // 7
        [Low,  Low,  Low,  Low,  Low,  Low,  Low ], // 8
        [Low,  Low,  Low,  High, High, Low,  Low ]  // 9
    ];

    let mut pins: [Pin<Output, Dynamic>; 7] = [
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade()
    ];

    let mut seven_seg_write = |digit: usize| {
        for (i, pin) in pins.iter_mut().enumerate() {
            pin.set_state(seven_seg_digits[digit][i]).unwrap();
        }
    };

    loop {
        for count in (0..10).rev() {
            delay_ms(1000);
            seven_seg_write(count);
        }
        delay_ms(3000);
    }
}