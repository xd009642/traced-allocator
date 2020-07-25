use std::alloc::System;
use traced_allocator::TracedAlloc;

#[global_allocator]
static GLOBAL: TracedAlloc<System> = TracedAlloc::new();

fn main() {
    println!("Creating vec");
    let mut v = vec![];

    println!("Pushing");
    for _ in 0..100 {
        v.push(false);
    }
    println!("Everything goes");
}
