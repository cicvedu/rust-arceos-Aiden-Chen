//! Simple memory allocation.
//!
//! TODO: more efficient

use core::{alloc::Layout, ptr::NonNull};
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator{
    start: usize,
    end: usize,
    next: usize,
    allocations: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start:0,
            end:0,
            next:0,
            allocations:0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.end = _start + _size;
        self.next = _start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!();
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
       let alloc_ptr = self.next;
       self.next = alloc_ptr + _layout.size();
       self.allocations += 1;
       Ok(unsafe { NonZeroUsize::new_unchecked(alloc_ptr) })
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end
    }

    fn used_bytes(&self) -> usize {
        self.next
    }

    fn available_bytes(&self) -> usize {
        self.end - self.next
    }
}
