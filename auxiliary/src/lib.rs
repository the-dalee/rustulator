//! Initialization code
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler
extern crate stm32f3xx_hal as hal;

mod led;
mod button;

pub use cortex_m_rt::entry;
pub use hal::{delay::Delay, prelude};

use hal::prelude::*;
use hal::gpio::{
    gpioe,
    Input,
    PullDown,
    gpiob::PB1,
};

use hal::stm32;

pub use led::Leds;
pub use button::Button;

pub fn init() -> (Leds, Button) {
    let dp = stm32::Peripherals::take().unwrap();

    &dp.GPIOA.bsrr.write(|w| w);

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));
    let button = Button::new(dp.GPIOA.split(&mut rcc.ahb));

    (leds, button)
}
