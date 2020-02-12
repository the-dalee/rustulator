macro_rules! gpio_block {
    ($block:ident, $p:ident, $($px:ident: $pxt:ident),* $(,)?) => {
        mod $p {
            use stm32f3xx_hal::gpio::*;
            use stm32f3xx_hal::gpio::$block::*;
            use super::Pin;

            pub fn from_parts(parts: Parts) -> (Regs, Pins) {
                let regs = Regs {
                    moder: parts.moder,
                    otyper: parts.otyper,
                    pupdr: parts.pupdr,
                };

                let pins = Pins {
                    $($px: parts.$px),*
                };

                (regs, pins)
            }

            pub struct Regs {
                pub moder: MODER,
                pub otyper: OTYPER,
                pub pupdr: PUPDR,
            }

            pub struct Pins {
                $(pub $px: $pxt<Input<Floating>>),*
            }

            $(
                impl<Mode> Pin for $pxt<Mode> {
                    fn pull_down_input(self, regs: &mut super::Regs) -> PXx<Input<PullDown>> {
                        self.into_pull_down_input(&mut regs.$p.moder, &mut regs.$p.pupdr)
                            .downgrade()
                            .downgrade()
                    }
                    fn push_pull_output(self, regs: &mut super::Regs) -> PXx<Output<PushPull>> {
                        self.into_push_pull_output(&mut regs.$p.moder, &mut regs.$p.otyper)
                            .downgrade()
                            .downgrade()
                    }
                }
            )*
        }
    }
}
