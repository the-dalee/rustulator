//! Uses the StatefulOutputPin embedded_hal trait to toggle the pin
//! On the stm32 discovery board this is the "south" led
//! Target board: STM32F3DISCOVERY

#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;

mod panicbit;

#[entry]
fn main() -> ! {
    panicbit::main()
}
