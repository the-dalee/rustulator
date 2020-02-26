use core::alloc::{Layout, GlobalAlloc};
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

pub struct Allocator {
    addr: AtomicUsize,
}

impl Allocator {
    pub const fn start_at(addr: usize) -> Self {
        Self {
            addr: AtomicUsize::new(addr),
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let padding = layout.size() % layout.align();
        let size = layout.size() + padding;
        let ptr = self.addr.fetch_add(size, Ordering::SeqCst);

        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}

unsafe impl Sync for Allocator {}