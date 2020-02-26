use core::alloc::{Layout, GlobalAlloc};
use core::sync::atomic::Ordering;
use core::cell::UnsafeCell;

pub struct Allocator {
    addr: UnsafeCell<usize>,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            addr: UnsafeCell::new(0),
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.addr.get();

        if *ptr == 0 {
            *ptr = cortex_m_rt::heap_start() as usize;
        }

        let padding = *ptr % layout.align();

        *ptr += padding;

        let res = *ptr;

        *ptr += layout.size();

        res as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}

unsafe impl Sync for Allocator {}