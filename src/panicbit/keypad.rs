use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::gpio::{Output, PXx, PushPull, Input, PullDown};
use enumflags2::BitFlags;
use super::gpio::Pin;

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

    pub fn poll(&mut self) -> BitFlags<Button> {
        let mut buttons = BitFlags::empty();

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

#[derive(BitFlags, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Button {
    N0 = 1,
    N1 = 1 << 1,
    N2 = 1 << 2,
    N3 = 1 << 3,
    N4 = 1 << 4,
    N5 = 1 << 5,
    N6 = 1 << 6,
    N7 = 1 << 7,
    N8 = 1 << 8,
    N9 = 1 << 9,
    A = 1 << 10,
    B = 1 << 11,
    C = 1 << 12,
    D = 1 << 13,
    ASTERISK = 1 << 14,
    HASH = 1 << 15,
}

impl Button {
    pub fn as_char(self) -> char {
        match self {
            Self::N0 => '0',
            Self::N1 => '1',
            Self::N2 => '2',
            Self::N3 => '3',
            Self::N4 => '4',
            Self::N5 => '5',
            Self::N6 => '6',
            Self::N7 => '7',
            Self::N8 => '8',
            Self::N9 => '9',
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::ASTERISK => '*',
            Self::HASH => '#',
        }
    }
}

pub const LAYOUT: [[Button; 4]; 4] = [
    [Button::N1, Button::N2, Button::N3, Button::A],
    [Button::N4, Button::N5, Button::N6, Button::B],
    [Button::N7, Button::N8, Button::N9, Button::C],
    [Button::ASTERISK, Button::N0, Button::HASH, Button::D],
];
