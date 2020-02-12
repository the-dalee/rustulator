use stm32f3xx_hal::gpio::*;
use stm32f3xx_hal::gpio::gpioa::*;
use stm32f3xx_hal::gpio::gpiob::*;
use stm32f3xx_hal::gpio::gpioc::*;
use stm32f3xx_hal::gpio::gpiod::*;
use stm32f3xx_hal::gpio::gpioe::*;
use stm32f3xx_hal::gpio::gpiof::*;

#[macro_use]
mod macros;

pub struct Regs {
    pub a: a::Regs,
    pub b: b::Regs,
    pub c: c::Regs,
    pub d: d::Regs,
    pub e: e::Regs,
    pub f: f::Regs,
}

pub struct Pins {
    pub pa0: PA0<Input<Floating>>,
    pub pa1: PA1<Input<Floating>>,
    pub pa2: PA2<Input<Floating>>,
    pub pa3: PA3<Input<Floating>>,
    pub pa4: PA4<Input<Floating>>,
    pub pa5: PA5<Input<Floating>>,
    pub pa6: PA6<Input<Floating>>,
    pub pa7: PA7<Input<Floating>>,
    pub pa8: PA8<Input<Floating>>,
    pub pa9: PA9<Input<Floating>>,
    pub pa10: PA10<Input<Floating>>,
    pub pa11: PA11<Input<Floating>>,
    pub pa12: PA12<Input<Floating>>,
    pub pa13: PA13<Input<Floating>>,
    pub pa14: PA14<Input<Floating>>,
    pub pa15: PA15<Input<Floating>>,
    pub pb0: PB0<Input<Floating>>,
    pub pb1: PB1<Input<Floating>>,
    pub pb2: PB2<Input<Floating>>,
    pub pb3: PB3<Input<Floating>>,
    pub pb4: PB4<Input<Floating>>,
    pub pb5: PB5<Input<Floating>>,
    pub pb6: PB6<Input<Floating>>,
    pub pb7: PB7<Input<Floating>>,
    pub pb8: PB8<Input<Floating>>,
    pub pb9: PB9<Input<Floating>>,
    pub pb10: PB10<Input<Floating>>,
    pub pb11: PB11<Input<Floating>>,
    pub pb12: PB12<Input<Floating>>,
    pub pb13: PB13<Input<Floating>>,
    pub pb14: PB14<Input<Floating>>,
    pub pb15: PB15<Input<Floating>>,
    pub pc0: PC0<Input<Floating>>,
    pub pc1: PC1<Input<Floating>>,
    pub pc2: PC2<Input<Floating>>,
    pub pc3: PC3<Input<Floating>>,
    pub pc4: PC4<Input<Floating>>,
    pub pc5: PC5<Input<Floating>>,
    pub pc6: PC6<Input<Floating>>,
    pub pc7: PC7<Input<Floating>>,
    pub pc8: PC8<Input<Floating>>,
    pub pc9: PC9<Input<Floating>>,
    pub pc10: PC10<Input<Floating>>,
    pub pc11: PC11<Input<Floating>>,
    pub pc12: PC12<Input<Floating>>,
    pub pc13: PC13<Input<Floating>>,
    pub pc14: PC14<Input<Floating>>,
    pub pc15: PC15<Input<Floating>>,
    pub pd0: PD0<Input<Floating>>,
    pub pd1: PD1<Input<Floating>>,
    pub pd2: PD2<Input<Floating>>,
    pub pd3: PD3<Input<Floating>>,
    pub pd4: PD4<Input<Floating>>,
    pub pd5: PD5<Input<Floating>>,
    pub pd6: PD6<Input<Floating>>,
    pub pd7: PD7<Input<Floating>>,
    pub pd8: PD8<Input<Floating>>,
    pub pd9: PD9<Input<Floating>>,
    pub pd10: PD10<Input<Floating>>,
    pub pd11: PD11<Input<Floating>>,
    pub pd12: PD12<Input<Floating>>,
    pub pd13: PD13<Input<Floating>>,
    pub pd14: PD14<Input<Floating>>,
    pub pd15: PD15<Input<Floating>>,
    pub pe0: PE0<Input<Floating>>,
    pub pe1: PE1<Input<Floating>>,
    pub pe2: PE2<Input<Floating>>,
    pub pe3: PE3<Input<Floating>>,
    pub pe4: PE4<Input<Floating>>,
    pub pe5: PE5<Input<Floating>>,
    pub pe6: PE6<Input<Floating>>,
    pub pe7: PE7<Input<Floating>>,
    pub pe8: PE8<Input<Floating>>,
    pub pe9: PE9<Input<Floating>>,
    pub pe10: PE10<Input<Floating>>,
    pub pe11: PE11<Input<Floating>>,
    pub pe12: PE12<Input<Floating>>,
    pub pe13: PE13<Input<Floating>>,
    pub pe14: PE14<Input<Floating>>,
    pub pe15: PE15<Input<Floating>>,
    pub pf0: PF0<Input<Floating>>,
    pub pf1: PF1<Input<Floating>>,
    pub pf2: PF2<Input<Floating>>,
    pub pf3: PF3<Input<Floating>>,
    pub pf4: PF4<Input<Floating>>,
    pub pf5: PF5<Input<Floating>>,
    pub pf6: PF6<Input<Floating>>,
    pub pf7: PF7<Input<Floating>>,
    pub pf8: PF8<Input<Floating>>,
    pub pf9: PF9<Input<Floating>>,
    pub pf10: PF10<Input<Floating>>,
    pub pf11: PF11<Input<Floating>>,
    pub pf12: PF12<Input<Floating>>,
    pub pf13: PF13<Input<Floating>>,
    pub pf14: PF14<Input<Floating>>,
    pub pf15: PF15<Input<Floating>>,
}

pub fn from_parts(
    gpioa: gpioa::Parts,
    gpiob: gpiob::Parts,
    gpioc: gpioc::Parts,
    gpiod: gpiod::Parts,
    gpioe: gpioe::Parts,
    gpiof: gpiof::Parts,
) -> (Regs, Pins) {
    let (regs_a, pins_a) = a::from_parts(gpioa);
    let (regs_b, pins_b) = b::from_parts(gpiob);
    let (regs_c, pins_c) = c::from_parts(gpioc);
    let (regs_d, pins_d) = d::from_parts(gpiod);
    let (regs_e, pins_e) = e::from_parts(gpioe);
    let (regs_f, pins_f) = f::from_parts(gpiof);

    let regs = Regs {
        a: regs_a,
        b: regs_b,
        c: regs_c,
        d: regs_d,
        e: regs_e,
        f: regs_f,
    };

    let pins = Pins {
        pa0: pins_a.pa0,
        pa1: pins_a.pa1,
        pa2: pins_a.pa2,
        pa3: pins_a.pa3,
        pa4: pins_a.pa4,
        pa5: pins_a.pa5,
        pa6: pins_a.pa6,
        pa7: pins_a.pa7,
        pa8: pins_a.pa8,
        pa9: pins_a.pa9,
        pa10: pins_a.pa10,
        pa11: pins_a.pa11,
        pa12: pins_a.pa12,
        pa13: pins_a.pa13,
        pa14: pins_a.pa14,
        pa15: pins_a.pa15,
        pb0: pins_b.pb0,
        pb1: pins_b.pb1,
        pb2: pins_b.pb2,
        pb3: pins_b.pb3,
        pb4: pins_b.pb4,
        pb5: pins_b.pb5,
        pb6: pins_b.pb6,
        pb7: pins_b.pb7,
        pb8: pins_b.pb8,
        pb9: pins_b.pb9,
        pb10: pins_b.pb10,
        pb11: pins_b.pb11,
        pb12: pins_b.pb12,
        pb13: pins_b.pb13,
        pb14: pins_b.pb14,
        pb15: pins_b.pb15,
        pc0: pins_c.pc0,
        pc1: pins_c.pc1,
        pc2: pins_c.pc2,
        pc3: pins_c.pc3,
        pc4: pins_c.pc4,
        pc5: pins_c.pc5,
        pc6: pins_c.pc6,
        pc7: pins_c.pc7,
        pc8: pins_c.pc8,
        pc9: pins_c.pc9,
        pc10: pins_c.pc10,
        pc11: pins_c.pc11,
        pc12: pins_c.pc12,
        pc13: pins_c.pc13,
        pc14: pins_c.pc14,
        pc15: pins_c.pc15,
        pd0: pins_d.pd0,
        pd1: pins_d.pd1,
        pd2: pins_d.pd2,
        pd3: pins_d.pd3,
        pd4: pins_d.pd4,
        pd5: pins_d.pd5,
        pd6: pins_d.pd6,
        pd7: pins_d.pd7,
        pd8: pins_d.pd8,
        pd9: pins_d.pd9,
        pd10: pins_d.pd10,
        pd11: pins_d.pd11,
        pd12: pins_d.pd12,
        pd13: pins_d.pd13,
        pd14: pins_d.pd14,
        pd15: pins_d.pd15,
        pe0: pins_e.pe0,
        pe1: pins_e.pe1,
        pe2: pins_e.pe2,
        pe3: pins_e.pe3,
        pe4: pins_e.pe4,
        pe5: pins_e.pe5,
        pe6: pins_e.pe6,
        pe7: pins_e.pe7,
        pe8: pins_e.pe8,
        pe9: pins_e.pe9,
        pe10: pins_e.pe10,
        pe11: pins_e.pe11,
        pe12: pins_e.pe12,
        pe13: pins_e.pe13,
        pe14: pins_e.pe14,
        pe15: pins_e.pe15,
        pf0: pins_f.pf0,
        pf1: pins_f.pf1,
        pf2: pins_f.pf2,
        pf3: pins_f.pf3,
        pf4: pins_f.pf4,
        pf5: pins_f.pf5,
        pf6: pins_f.pf6,
        pf7: pins_f.pf7,
        pf8: pins_f.pf8,
        pf9: pins_f.pf9,
        pf10: pins_f.pf10,
        pf11: pins_f.pf11,
        pf12: pins_f.pf12,
        pf13: pins_f.pf13,
        pf14: pins_f.pf14,
        pf15: pins_f.pf15,
    };

    (regs, pins)
}

gpio_block!(
    gpioa, a,
    pa0: PA0,
    pa1: PA1,
    pa2: PA2,
    pa3: PA3,
    pa4: PA4,
    pa5: PA5,
    pa6: PA6,
    pa7: PA7,
    pa8: PA8,
    pa9: PA9,
    pa10: PA10,
    pa11: PA11,
    pa12: PA12,
    pa13: PA13,
    pa14: PA14,
    pa15: PA15,
);

gpio_block!(
    gpiob, b,
    pb0: PB0,
    pb1: PB1,
    pb2: PB2,
    pb3: PB3,
    pb4: PB4,
    pb5: PB5,
    pb6: PB6,
    pb7: PB7,
    pb8: PB8,
    pb9: PB9,
    pb10: PB10,
    pb11: PB11,
    pb12: PB12,
    pb13: PB13,
    pb14: PB14,
    pb15: PB15,
);

gpio_block!(
    gpioc, c,
    pc0: PC0,
    pc1: PC1,
    pc2: PC2,
    pc3: PC3,
    pc4: PC4,
    pc5: PC5,
    pc6: PC6,
    pc7: PC7,
    pc8: PC8,
    pc9: PC9,
    pc10: PC10,
    pc11: PC11,
    pc12: PC12,
    pc13: PC13,
    pc14: PC14,
    pc15: PC15,
);

gpio_block!(
    gpiod, d,
    pd0: PD0,
    pd1: PD1,
    pd2: PD2,
    pd3: PD3,
    pd4: PD4,
    pd5: PD5,
    pd6: PD6,
    pd7: PD7,
    pd8: PD8,
    pd9: PD9,
    pd10: PD10,
    pd11: PD11,
    pd12: PD12,
    pd13: PD13,
    pd14: PD14,
    pd15: PD15,
);

gpio_block!(
    gpioe, e,
    pe0: PE0,
    pe1: PE1,
    pe2: PE2,
    pe3: PE3,
    pe4: PE4,
    pe5: PE5,
    pe6: PE6,
    pe7: PE7,
    pe8: PE8,
    pe9: PE9,
    pe10: PE10,
    pe11: PE11,
    pe12: PE12,
    pe13: PE13,
    pe14: PE14,
    pe15: PE15,
);

gpio_block!(
    gpiof, f,
    pf0: PF0,
    pf1: PF1,
    pf2: PF2,
    pf3: PF3,
    pf4: PF4,
    pf5: PF5,
    pf6: PF6,
    pf7: PF7,
    pf8: PF8,
    pf9: PF9,
    pf10: PF10,
    pf11: PF11,
    pf12: PF12,
    pf13: PF13,
    pf14: PF14,
    pf15: PF15,
);

pub trait Pin {
    fn pull_down_input(self, regs: &mut Regs) -> PXx<Input<PullDown>>;
    fn push_pull_output(self, regs: &mut Regs) -> PXx<Output<PushPull>>;
}
