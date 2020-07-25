use std::alloc::{GlobalAlloc, Layout};
use std::panic::Location;

pub struct TracedAlloc<Inner: GlobalAlloc> {
    pub allocator: Inner
}

unsafe impl<T> GlobalAlloc for TracedAlloc<T> where T: GlobalAlloc {
    #[track_caller]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("Alloc {:?} at {:?}", Location::caller(), layout);
        self.allocator.alloc(layout) 
    }

    #[track_caller]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        print!("Dealloc {:?} at {:?}", Location::caller(), layout);
        self.allocator.dealloc(ptr, layout)
    }
}

