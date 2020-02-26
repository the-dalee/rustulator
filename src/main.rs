//! Uses the StatefulOutputPin embedded_hal trait to toggle the pin
//! On the stm32 discovery board this is the "south" led
//! Target board: STM32F3DISCOVERY

#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate panic_semihosting;
extern crate alloc;

use cortex_m_rt::entry;
use core::alloc::Layout;

mod panicbit;

#[global_allocator]
static ALLOCATOR: panicbit::Allocator = panicbit::Allocator::new();

#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
    panic!("Allocation error");
}

#[entry]
fn main() -> ! {
    panicbit::main()
}
