# traced-allocator

Work on tracing memory allocations in an application via location.

This is a very WIP thing.

## Current output

```
Alloc Layout { size_: 4, align_: 1 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 5, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 197, col: 37 }
Dealloc Layout { size_: 4, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 203, col: 22 }
Alloc Layout { size_: 40, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 48, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 80, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 8, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 24, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 64, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 1024, align_: 1 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 104, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 8, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Creating vec
Pushing
Alloc Layout { size_: 8, align_: 1 } at Location { file: "src/main.rs", line: 4, col: 1 }
Alloc Layout { size_: 16, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 197, col: 37 }
Dealloc Layout { size_: 8, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 203, col: 22 }
Alloc Layout { size_: 32, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 197, col: 37 }
Dealloc Layout { size_: 16, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 203, col: 22 }
Alloc Layout { size_: 64, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 197, col: 37 }
Dealloc Layout { size_: 32, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 203, col: 22 }
Alloc Layout { size_: 128, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 197, col: 37 }
Dealloc Layout { size_: 64, align_: 1 } at Location { file: "/rustc/e51b714db8ff82ac38ea7c6742d6f5480e2e77bd/src/libcore/alloc/global.rs", line: 203, col: 22 }
Everything goes
Dealloc Layout { size_: 128, align_: 1 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 1024, align_: 1 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 104, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 8, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 8, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 64, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 24, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 5, align_: 1 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 40, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 48, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
Dealloc Layout { size_: 80, align_: 8 } at Location { file: "src/main.rs", line: 4, col: 1 }
```
