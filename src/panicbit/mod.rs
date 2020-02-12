mod keypad;
use keypad::{Keypad, Button};

use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::stm32;
use cortex_m::iprintln;

pub fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let stim = &mut cp.ITM.stim[0];

    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut keypad = Keypad::new(
        gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).downgrade().downgrade(),
        gpioa.pa2.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).downgrade().downgrade(),
        gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).downgrade().downgrade(),
        gpioa.pa6.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper).downgrade().downgrade(),
        gpioc.pc4.into_pull_down_input(&mut gpioc.moder, &mut gpioc.pupdr).downgrade().downgrade(),
        gpiob.pb0.into_pull_down_input(&mut gpiob.moder, &mut gpiob.pupdr).downgrade().downgrade(),
        gpiob.pb2.into_pull_down_input(&mut gpiob.moder, &mut gpiob.pupdr).downgrade().downgrade(),
        gpioe.pe8.into_pull_down_input(&mut gpioe.moder, &mut gpioe.pupdr).downgrade().downgrade(),
    );

    let mut led = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {
        let buttons = keypad.poll();

        if buttons.contains(Button::N5) {
            led.set_low().unwrap();
        } else {
            led.set_high().unwrap();
        }

        iprintln!(stim, "buttons: {:#?}", buttons);

        cortex_m::asm::delay(1_000);
    }
}
