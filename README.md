# traced-allocator

Work on tracing memory allocations in an application via location.

This is a very WIP thing.

## Current output

```

Alloc Layout { size_: 4, align_: 1 } at Location { file: "src/main.rs", line: 4, col: 1 }
realloc Layout { size_: 4, align_: 1 } to 5 at Location { file: "src/main.rs", line: 4, col: 1 }
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
realloc Layout { size_: 8, align_: 1 } to 16 at Location { file: "src/main.rs", line: 4, col: 1 }
realloc Layout { size_: 16, align_: 1 } to 32 at Location { file: "src/main.rs", line: 4, col: 1 }
realloc Layout { size_: 32, align_: 1 } to 64 at Location { file: "src/main.rs", line: 4, col: 1 }
realloc Layout { size_: 64, align_: 1 } to 128 at Location { file: "src/main.rs", line: 4, col: 1 }
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
