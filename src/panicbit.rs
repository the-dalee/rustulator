use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::gpio::{PXx, PushPull, Output};
use enumflags2::BitFlags;

#[macro_use]
mod macros;

mod gpio;
use gpio::Pin;

mod peripherals;
use peripherals::Peripherals;

mod keypad;
use keypad::{Button, Keypad};

mod allocator;
pub use allocator::Allocator;

mod lcd;
use lcd::Lcd;

use alloc::string::String;
use alloc::vec::Vec;

pub fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut p = Peripherals::take().unwrap();

    let stim = &mut cp.ITM.stim[0];

    let mut keypad = Keypad::new(
        &mut p.gpio_regs,
        p.pins.pb11,
        p.pins.pb13,
        p.pins.pb15,
        p.pins.pd9,
        p.pins.pd11,
        p.pins.pd13,
        p.pins.pd15,
        p.pins.pc6,
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

    let mut lcd = Lcd::new(
        &mut p.gpio_regs,
        p.pins.pa8,
        p.pins.pa10,
        p.pins.pb10,
        p.pins.pb12,
        p.pins.pb14,
        p.pins.pd8,
        p.pins.pd10,
        p.pins.pd12,
        p.pins.pd14,
        p.pins.pc7,
    );

    lcd.set_register_select(false);

    // Clear display
    lcd.send_data(0b0000_0001);
    // Return home
    lcd.send_data(0b0000_0010);
    // Entry mode set
    lcd.send_data(0b0000_0110);
    // Display on/off
    lcd.send_data(0b0000_1111);
    // Function Set
    lcd.send_data(0b0011_1000);

    lcd.set_register_select(true);

    lcd.send_data(b'R');
    lcd.send_data(b'u');
    lcd.send_data(b's');
    lcd.send_data(b't');

    let mut sequences = Vec::new();
    let mut sequence = String::new();
    let mut previous_buttons = BitFlags::empty();

    loop {
        let buttons = keypad.poll();

        if buttons == previous_buttons {
            continue;
        }

        let mut diff_buttons = buttons;
        diff_buttons.remove(previous_buttons);

        previous_buttons = buttons;

        for button in diff_buttons.iter() {
            let button = button.as_char();

            match button {
                '#' => {
                    println!(stim, "Storing sequence {}", sequence);
                    sequences.push(sequence);
                    sequence = String::new();
                },
                '*' => println!(stim, "Stored sequences: {:#?}", sequences),
                _ => {
                    lcd.send_data(button as u8);
                    println!(stim, "button: {}", button);

                    sequence.push(button);
                    println!(stim, "sequence: {}", sequence);
                }
            }

        }

        display_buttons_via_leds(buttons, &mut leds);
    }
}

fn display_buttons_via_leds(buttons: BitFlags<Button>, leds: &mut [PXx<Output<PushPull>>]) {
    // disable all leds
    for led in &mut *leds {
        led.set_low().ok();
    }

    // map button row and column to leds
    for button in buttons.iter() {
        let (row, column) = button.row_and_column();

        leds[row].set_high().ok();
        leds[4 + column].set_high().ok();
    }
}
