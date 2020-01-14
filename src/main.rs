#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};
use itertools::Itertools;

#[entry]
fn main() -> ! {
    let (mut leds, mut button) = aux5::init();
    // let half_period = 500_u16;

    // let delay = Delay::new(syst: SYST, clocks: Clocks);

    // (0..leds.len())
    // .cycle()
    // .tuple_windows()
    // .for_each(|(current, next)| {
    //     leds[next].on();
    //     delay.delay_ms(half_period);
    //     leds[current].off();
    //     delay.delay_ms(half_period);
    // });

    loop {
        for i in 0..8 {
            if button.is_pressed() {
                leds[i].on();
            } else {
                leds[i].off();
            }
        }
    }
}