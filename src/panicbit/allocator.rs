use core::alloc::{Layout, GlobalAlloc};
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use core::cell::UnsafeCell;

pub struct Allocator {
    addr: UnsafeCell<usize>,
}

impl Allocator {
    pub const fn start_at(addr: usize) -> Self {
        Self {
            addr: UnsafeCell::new(addr),
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.addr.get();
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