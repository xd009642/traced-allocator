use libc_print::libc_println;
use std::alloc::{GlobalAlloc, Layout, System};
use std::panic::Location;

pub struct TracedAlloc<T: GlobalAlloc> {
    pub allocator: T,
}

impl TracedAlloc<System> {
    pub const fn new() -> Self {
        Self { allocator: System }
    }
}

unsafe impl<T> GlobalAlloc for TracedAlloc<T>
where
    T: GlobalAlloc,
{
    #[track_caller]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc_println!("Alloc {:?} at {:?}", layout, Location::caller());
        self.allocator.alloc(layout)
    }

    #[track_caller]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        libc_println!("Dealloc {:?} at {:?}", layout, Location::caller());
        self.allocator.dealloc(ptr, layout)
    }
}
