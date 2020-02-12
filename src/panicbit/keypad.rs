use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::gpio::{Output, PXx, PushPull, Input, PullDown};
use bitflags::bitflags;
use super::gpio::Pin;

bitflags! {
    pub struct Button: u16 {
        const N0 = 1;
        const N1 = 1 << 1;
        const N2 = 1 << 2;
        const N3 = 1 << 3;
        const N4 = 1 << 4;
        const N5 = 1 << 5;
        const N6 = 1 << 6;
        const N7 = 1 << 7;
        const N8 = 1 << 8;
        const N9 = 1 << 9;
        const A = 1 << 10;
        const B = 1 << 11;
        const C = 1 << 12;
        const D = 1 << 13;
        const ASTERISK = 1 << 14;
        const HASH = 1 << 15;
    }
}

pub const LAYOUT: [[Button; 4]; 4] = [
    [Button::N1, Button::N2, Button::N3, Button::A],
    [Button::N4, Button::N5, Button::N6, Button::B],
    [Button::N7, Button::N8, Button::N9, Button::C],
    [Button::ASTERISK, Button::N0, Button::HASH, Button::D],
];

pub struct Keypad {
    pub rows: [PXx<Input<PullDown>>; 4],
    pub cols: [PXx<Output<PushPull>>; 4],
}

impl Keypad {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        regs: &mut super::gpio::Regs,
        pin1: impl Pin,
        pin2: impl Pin,
        pin3: impl Pin,
        pin4: impl Pin,
        pin5: impl Pin,
        pin6: impl Pin,
        pin7: impl Pin,
        pin8: impl Pin,
    ) -> Self {
        Self {
            rows: [
                pin8.pull_down_input(regs),
                pin7.pull_down_input(regs),
                pin6.pull_down_input(regs),
                pin5.pull_down_input(regs),
            ],
            cols: [
                pin4.push_pull_output(regs),
                pin3.push_pull_output(regs),
                pin2.push_pull_output(regs),
                pin1.push_pull_output(regs),
            ],
        }
    }

    pub fn poll(&mut self) -> Button {
        let mut buttons = Button::empty();

        for col in &mut self.cols {
            col.set_low().ok();
        }

        for (row_pin, row) in self.rows.iter_mut().zip(&LAYOUT) {
            for (col_pin, button) in self.cols.iter_mut().zip(row) {
                col_pin.set_high().ok();

                if row_pin.is_high().unwrap_or(false) {
                    buttons |= *button;
                }

                col_pin.set_low().ok();
            }
        }

        buttons
    }
}
