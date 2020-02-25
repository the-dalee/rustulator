use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::gpio::{PXx, PushPull, Output};
use cortex_m::iprintln;
use enumflags2::BitFlags;

mod gpio;
use gpio::Pin;

mod peripherals;
use peripherals::Peripherals;

mod keypad;
use keypad::{Button, Keypad};

mod lcd;
use lcd::Lcd;

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

    let mut lcd = Lcd::new(
        &mut p.gpio_regs,
        p.pins.pa8,
        p.pins.pa10,
        p.pins.pd0,
        p.pins.pc12, // p.pins.pd1,
        p.pins.pd2,
        p.pins.pd3,
        p.pins.pd4,
        p.pins.pd5,
        p.pins.pd6,
        p.pins.pd7,
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
            lcd.send_data(button as u8);
            iprintln!(stim, "button: {}", button);
        }

        display_buttons_via_leds(buttons, &mut leds);
    }
}

fn display_buttons_via_leds(buttons: BitFlags<Button>, leds: &mut [PXx<Output<PushPull>>]) {
    for led in &mut *leds {
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
}