#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{delay_ms, delay_us};
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;
use core::cell;
use panic_halt as _;
use embedded_hal::serial::Read;
use pwm_pca9685::nb::Error;
use crate::Duration::{Eighth, Quarter};
use crate::Note::*;

#[derive(PartialEq)]
enum Note {
    Rest = 0,
    B0 = 31,
    C1 = 33,
    CS1 = 35,
    D1 = 37,
    DS1 = 39,
    E1 = 41,
    F1 = 44,
    FS1 = 46,
    G1 = 49,
    GS1 = 52,
    A1 = 55,
    AS1 = 58,
    B1 = 62,
    C2 = 65,
    CS2 = 69,
    D2 = 73,
    DS2 = 78,
    E2 = 82,
    F2 = 87,
    FS2 = 93,
    G2 = 98,
    GS2 = 104,
    A2 = 110,
    AS2 = 117,
    B2 = 123,
    C3 = 131,
    CS3 = 139,
    D3 = 147,
    DS3 = 156,
    E3 = 165,
    F3 = 175,
    FS3 = 185,
    G3 = 196,
    GS3 = 208,
    A3 = 220,
    AS3 = 233,
    B3 = 247,
    C4 = 262,
    CS4 = 277,
    D4 = 294,
    DS4 = 311,
    E4 = 330,
    F4 = 349,
    FS4 = 370,
    G4 = 392,
    GS4 = 415,
    A4 = 440,
    AS4 = 466,
    B4 = 494,
    C5 = 523,
    CS5 = 554,
    D5 = 587,
    DS5 = 622,
    E5 = 659,
    F5 = 698,
    FS5 = 740,
    G5 = 784,
    GS5 = 831,
    A5 = 880,
    AS5 = 932,
    B5 = 988,
    C6 = 1047,
    CS6 = 1109,
    D6 = 1175,
    DS6 = 1245,
    E6 = 1319,
    F6 = 1397,
    FS6 = 1480,
    G6 = 1568,
    GS6 = 1661,
    A6 = 1760,
    AS6 = 1865,
    B6 = 1976,
    C7 = 2093,
    CS7 = 2217,
    D7 = 2349,
    DS7 = 2489,
    E7 = 2637,
    F7 = 2794,
    FS7 = 2960,
    G7 = 3136,
    GS7 = 3322,
    A7 = 3520,
    AS7 = 3729,
    B7 = 3951,
    C8 = 4186,
    CS8 = 4435,
    D8 = 4699,
    DS8 = 4978
}

enum Duration {
    Full = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16
}

const MELODY: [(Note, Duration); 8] = [
    (C4, Quarter),
    (G3, Eighth),
    (G3, Eighth),
    (A3, Quarter),
    (G3, Quarter),
    (Rest, Quarter),
    (B3, Quarter),
    (C4, Quarter)
];

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut output = pins.d8.into_output().downgrade();

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    // let mut led = pins.d13.into_output();
    loop {
        for (note, duration) in &MELODY {
            tone(&mut serial, &mut output, note, 1000 / *duration as u32);
        }
    }
}

/// A super hacky version of the tone() function from the Arduino SDK using simple delays. A better
/// version of this would probably utilize interrupts, which I'll have to figure out later.
fn tone<T: core::fmt::Debug>(writer: &mut dyn ufmt::uWrite<Error=T>, output: &mut Pin<Output, Dynamic>, frequency: &Note, duration_in_millis: u32) {
    if *frequency == Rest {
        delay_ms(duration_in_millis as u16);
        return
    }

    let period_in_micros = 1_000_000 / *frequency as u32;
    let half_wave = period_in_micros / 2;

    let iterations = duration_in_millis * 1000 / period_in_micros;
    ufmt::uwriteln!(writer, "Half wave: '{}' for iterations: '{}'", half_wave, iterations).unwrap();

    for _i in 0..iterations {
        output.set_high();
        delay_us(half_wave);
        output.set_low();
        delay_us(half_wave);
    }
}
