use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::gpio::{Output, PXx, PushPull, Input, PullDown};
use super::gpio::Pin;

pub const NUM_DELAY_CYCLES: u32 = 50_000;

pub struct Lcd {
    pub register_select: PXx<Output<PushPull>>,
    pub enable: PXx<Output<PushPull>>,
    pub data0: PXx<Output<PushPull>>,
    pub data1: PXx<Output<PushPull>>,
    pub data2: PXx<Output<PushPull>>,
    pub data3: PXx<Output<PushPull>>,
    pub data4: PXx<Output<PushPull>>,
    pub data5: PXx<Output<PushPull>>,
    pub data6: PXx<Output<PushPull>>,
    pub data7: PXx<Output<PushPull>>,
}

impl Lcd {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        regs: &mut super::gpio::Regs,
        register_select: impl Pin,
        enable: impl Pin,
        data0: impl Pin,
        data1: impl Pin,
        data2: impl Pin,
        data3: impl Pin,
        data4: impl Pin,
        data5: impl Pin,
        data6: impl Pin,
        data7: impl Pin,
    ) -> Self {
        Self {
            register_select: register_select.push_pull_output(regs),
            enable: enable.push_pull_output(regs),
            data0: data0.push_pull_output(regs),
            data1: data1.push_pull_output(regs),
            data2: data2.push_pull_output(regs),
            data3: data3.push_pull_output(regs),
            data4: data4.push_pull_output(regs),
            data5: data5.push_pull_output(regs),
            data6: data6.push_pull_output(regs),
            data7: data7.push_pull_output(regs),
        }
    }

    pub fn set_register_select(&mut self, enabled: bool) {
        if enabled {
            self.register_select.set_high().ok();
        } else {
            self.register_select.set_low().ok();
        }
        cortex_m::asm::delay(NUM_DELAY_CYCLES);
    }

    pub fn set_enable(&mut self, enabled: bool) {
        if enabled {
            self.enable.set_high().ok();
        } else {
            self.enable.set_low().ok();
        }
        cortex_m::asm::delay(NUM_DELAY_CYCLES);
    }

    pub fn send_data(&mut self, data: u8) {
        self.set_enable(true);
        cortex_m::asm::delay(NUM_DELAY_CYCLES);
        self.set_data(data);
        cortex_m::asm::delay(NUM_DELAY_CYCLES);
        self.set_enable(false);
        cortex_m::asm::delay(NUM_DELAY_CYCLES);
    }

    pub fn set_data(&mut self, data: u8) {
        if (data & (1 << 0)) == 0 {
            self.data0.set_low().ok();
        } else {
            self.data0.set_high().ok();
        }

        if (data & (1 << 1)) == 0 {
            self.data1.set_low().ok();
        } else {
            self.data1.set_high().ok();
        }

        if (data & (1 << 2)) == 0 {
            self.data2.set_low().ok();
        } else {
            self.data2.set_high().ok();
        }

        if (data & (1 << 3)) == 0 {
            self.data3.set_low().ok();
        } else {
            self.data3.set_high().ok();
        }

        if (data & (1 << 4)) == 0 {
            self.data4.set_low().ok();
        } else {
            self.data4.set_high().ok();
        }

        if (data & (1 << 5)) == 0 {
            self.data5.set_low().ok();
        } else {
            self.data5.set_high().ok();
        }

        if (data & (1 << 6)) == 0 {
            self.data6.set_low().ok();
        } else {
            self.data6.set_high().ok();
        }

        if (data & (1 << 7)) == 0 {
            self.data7.set_low().ok();
        } else {
            self.data7.set_high().ok();
        }
    }
}