use std::alloc::System;
use traced_allocator::TracedAlloc;

#[global_allocator]
static A: TracedAlloc<System> = TracedAlloc { allocator: System };

fn main() {
    println!("Creating vec");
    let mut v = vec![];

    println!("Pushing");
    for _ in 0..100 {
        v.push(false);
    }
    println!("Everything goes");
}
