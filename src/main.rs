#![no_std]
#![no_main]

use core::fmt::Write;
use ssd1306::{mode::TerminalMode, prelude::*, I2CDisplayInterface, Ssd1306};
use aht10::AHT10;
use arduino_hal::Delay;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial =
        arduino_hal::Usart::new(dp.USART0, pins.d0, pins.d1.into_output(), 9600.into());

    let mut led = pins.d13.into_output();

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_terminal_mode();

    display.init().unwrap();
    display.clear().unwrap();

    for c in "c suger".chars() {
        display.print_char(c).unwrap();
    }

    display.set_position(0, 1).unwrap();

    for c in "rust bra".chars() {
        display.print_char(c).unwrap();
    }

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
