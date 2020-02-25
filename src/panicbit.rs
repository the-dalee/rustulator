mod gpio;
use gpio::Pin;

mod peripherals;
use peripherals::Peripherals;

mod keypad;
use keypad::{Keypad, Button};

use stm32f3xx_hal::prelude::*;
use cortex_m::iprintln;

pub fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut p = Peripherals::take().unwrap();

    let stim = &mut cp.ITM.stim[0];

    let mut keypad = Keypad::new(
        &mut p.gpio_regs,
        p.pins.pd8,
        p.pins.pd9,
        p.pins.pd10,
        p.pins.pd11,
        p.pins.pd12,
        p.pins.pd13,
        p.pins.pd14,
        p.pins.pd15,
    );

    let mut leds = [
        p.pins.pe8.push_pull_output(&mut p.gpio_regs),
        p.pins.pe9.push_pull_output(&mut p.gpio_regs),
        p.pins.pe10.push_pull_output(&mut p.gpio_regs),
        p.pins.pe11.push_pull_output(&mut p.gpio_regs),
        p.pins.pe12.push_pull_output(&mut p.gpio_regs),
        p.pins.pe13.push_pull_output(&mut p.gpio_regs),
        p.pins.pe14.push_pull_output(&mut p.gpio_regs),
        p.pins.pe15.push_pull_output(&mut p.gpio_regs),
    ];

    let mut previous_buttons = Button::empty();

    loop {
        let buttons = keypad.poll();

        if buttons == previous_buttons {
            continue;
        }

        previous_buttons = buttons;

        for led in &mut leds {
            led.set_low().ok();
        }

        for (row_i, row) in keypad::LAYOUT.iter().enumerate() {
            for (col_i, button) in row.iter().enumerate() {
                if !buttons.contains(*button) {
                    continue
                }

                leds[row_i].set_high().ok();
                leds[4 + col_i].set_high().ok();
            }
        }

        // iprintln!(stim, "buttons: {:#?}", buttons);
    }
}
