OSEPP Arduino Basics in Rust 🦀
==================

## Background

These projects served as a way to introduce myself to embedded Rust and 
re-introduce myself to Arduino programming. Each of these is a translation of
a project in the long discontinued 
[OSEPP Arduino Basics Starter Kit](https://www.osepp.com/starter-kits/86-osepp-101-arduino-basics-starter-kit).
There is nothing unique about this kit except that I already had it sitting at
my desk drawer. Most of these projects are similar to examples in the Arduino
IDE, and they should work on any Arduino hardware.

These projects were created using the excellent
[Rahix/avr-hal-template](https://github.com/Rahix/avr-hal-template). This 
template is very simple, and highly recommended for quickly getting started
with Rust programming on Arduino.

## General Observations

All of these projects were translated from Arduino sketches provided with the 
OSEPP kit. In many cases, the original C code could be translated almost verbatim. 
However, there are some differences in the way that the hardware is exposed in
`arduino-hal` compared to the original SDK. Most notably, all pins are accessed
via struct fields, rather than simple `byte` or `int` parameters on function
calls. The obvious benefit here is the additional safety provided at compile
time. In some cases though, pins are iterated. Since the fields cannot be
iterated directly, the pins are placed into an array and the array is 
iterated.

## Project Descriptions

Below is a description of each project, as well as a link to the corresponding
OSEPP tutorial which contains a more in-depth description as well as schematics
and wiring diagram.

* [tutorial2-digital-outputs]([tutorial2-digital-outputs]) ([docs]((https://www.osepp.com/tutorial-2-controlling-digital-outputs))) - 
Use digital output signals to control an array of 4 LEDs.
* [tutorial3-digital-input](tutorial3-digital-input) ([docs](https://www.osepp.com/tutorial-3-using-digital-input)) -
Use digital input to read the state of a tact switch and report over the serial port monitor.
* [tutorial4-led-game](tutorial4-led-game) ([docs](https://www.osepp.com/tutorial-4-an-led-game)) -
Combine digital input and output signals to create a simple game using LEDs and a tact switch.
* [tutorial5-voltage-meter](tutorial5-voltage-meter) ([docs](https://www.osepp.com/tutorial-5-building-voltage-meter)) -
Build a simple voltage divider circuit and report output voltage over the serial port monitor.
* [tutorial6-buzzer-melody](tutorial6-buzzer-melody) ([docs](https://www.osepp.com/tutorial-6-using-buzzer-to-play-a-melody)) -
Use digital output and a buzzer to play a melody.
  * **Note:** The original implementation for this circuit relies on the 
  `tone()` function in the Arduino SDK. Rather than attempt to re-implement
  the tone function, it is simulated using `delay_us()`. In the future, it 
  might be interested to try implementing the tone function using interrupts
  in Rust.
* [tutorial7-7segment-countdown](tutorial7-7segement-countdown) ([docs](https://www.osepp.com/tutorial-7-counting-down-with-a-7-segment-led)) -
Use digital outputs to display a countdown from 9 to 0 on a 7 segment LED display

## Usage
If you don't have it already, install  [`ravedude`]:

```bash
cargo install ravedude
```

These projects are configured to target the [OSEPP Uno R3 Plus](https://www.osepp.com/electronic-modules/microcontroller-boards/105-osepp-uno-r3-plus)
board which uses the old nano bootloader. To target different hardware, you 
will need to change the features on the `arduino-hal` and `avr-device` 
dependencies.

```toml
[dependencies.arduino-hal]
git = "https://github.com/rahix/avr-hal"
rev = "533159f6c6a508abe4ecec34bf5013d7a1eb0cf5"
features = ["arduino-nano"]

[dependencies.avr-device]
version = "0.4"
features = ["atmega328p", "rt"]
```

**Note:** Most projects only depend on `arduino-hal`. The following options
are supported by arduino-hal at this time. If using a different board, replace
the `arduino-nano` feature string with the string corresponding to your board.

* "Adafruit Trinket" - `trinket`
* "Adafruit Trinket Pro" - `trinket-pro`
* "Arduino Leonardo" - `arduino-leonardo`
* "Arduino Mega 2560" - `arduino-mega2560`
* "Arduino Mega 1280" - `arduino-mega1280`
* "Arduino Nano" - `arduino-nano`
* "Arduino Nano New Bootloader" - `arduino-nano`
* "Arduino Uno" - `arduino-uno`
* "SparkFun ProMicro" - `sparkfun-promicro`
* "Nano168" - `nano168`

Valid options for the chip feature in `avr-device` are as follows:

* `atmega32u4`
* `atmega48p`
* `atmega168`
* `atmega328p`
* `atmega1280`
* `atmega2560`
* `attiny85`
* `attiny88`

To run the projects, you will need to set the `RAVEDUDE_PORT` environment 
variable to the serial port on your system. 
See the [ravedude README](https://github.com/Rahix/avr-hal/tree/main/ravedude#ravedude-)
for more advanced configuration options. Finding the serial port on your system
is outside the scope of this document. In general, USB serial ports on
linux will start with `/dev/ttyUSB` and on macOS will start with `/dev/tty.usbserial`. 

For example, to run the `tutorial2-digital-outputs` project: 

```bash
cd tutorial2-digital-outputs
export RAVEDUDE_PORT=/dev/tty.usbserial-ABC123
cargo run
```
[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[`ravedude`]: https://github.com/Rahix/avr-hal/tree/next/ravedude

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution
This repo was created simply to document a personal project, but feel free to 
submit issues/MRs to address any errors found.