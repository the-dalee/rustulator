use core::ops;
use hal::prelude::*;
use hal::gpio::gpioe::{self, PEx};
use hal::gpio::{Output, PushPull};
use ::hal::hal::digital::v2::OutputPin;

pub struct Led {
    pex: PEx<Output<PushPull>>,
}

pub struct Leds {
    leds: [Led; 8],
}

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pex.set_low();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pex.set_high();
    }
}

impl Leds {
    pub fn new(mut gpio_e: gpioe::Parts) -> Self {
        let n = gpio_e
            .pe9
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        let ne = gpio_e
            .pe10
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        let e = gpio_e
            .pe11
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        let se = gpio_e
            .pe12
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        let s = gpio_e
            .pe13
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        let sw = gpio_e
            .pe14
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        let w = gpio_e
            .pe15
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
        let nw = gpio_e
            .pe8
            .into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);

        Leds {
            leds: [
                Led { pex: n.downgrade() },
                Led { pex: ne.downgrade() },
                Led { pex: e.downgrade() },
                Led { pex: se.downgrade() },
                Led { pex: s.downgrade() },
                Led { pex: sw.downgrade() },
                Led { pex: w.downgrade() },
                Led { pex: nw.downgrade() },
            ],
        }
    }
}

impl ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}
