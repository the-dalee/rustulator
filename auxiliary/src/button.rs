use core::ops;
use hal::prelude::*;
use hal::gpio::gpioa::{self, PA0};
use hal::gpio::{Input, Output, PushPull, PullDown, PullUp};
use ::hal::hal::digital::v2::InputPin;


pub struct Button {
    inner: PA0<Input<PullDown>>,
}

impl Button {
    pub fn new(mut parts: gpioa::Parts) -> Self {
        let button = parts.pa0.into_pull_down_input(&mut parts.moder, &mut parts.pupdr);

        Self {
            inner: button
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.inner.is_high().unwrap()
    }
}
