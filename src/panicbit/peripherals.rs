use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::{rcc::Rcc, stm32};
use super::gpio;

pub struct Peripherals {
    pub rcc: Rcc,
    pub gpio_regs: gpio::Regs,
    pub pins: gpio::Pins,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        stm32::Peripherals::take().map(|p| {
            let mut rcc = p.RCC.constrain();

            let (gpio_regs, pins) = gpio::from_parts(
                p.GPIOA.split(&mut rcc.ahb),
                p.GPIOB.split(&mut rcc.ahb),
                p.GPIOC.split(&mut rcc.ahb),
                p.GPIOD.split(&mut rcc.ahb),
                p.GPIOE.split(&mut rcc.ahb),
                p.GPIOF.split(&mut rcc.ahb),
            );

            Self {
                gpio_regs,
                pins,
                rcc,
            }
        })
    }
}