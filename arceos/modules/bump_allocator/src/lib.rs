#![no_std]

use core::{ptr::NonNull, usize};

use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};
// use log::warn;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE:usize> {
    start: usize,
    end: usize,
    byte_used: usize,
    next: usize,
    page_next:usize,
}

impl<const PAGE_SIZE:usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new()->Self {
        Self { start: 0, end:0, byte_used:0,next:0, page_next:0}
    }
}

impl<const PAGE_SIZE:usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.next = start;
        self.page_next = self.end;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        Err(AllocError::NoMemory)
    }
}

impl<const PAGE_SIZE:usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: core::alloc::Layout) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        // warn!("alloc {}", layout.size());
        let p = self.next;
        let next = self.next + layout.size() + layout.align() - 1;
        let mask = usize::MAX << layout.align().trailing_zeros();
        let next = next & mask;
        if next > self.page_next {
            return Err(AllocError::NoMemory);
        }
        self.next = next;
        self.byte_used += layout.size();
        let ptr = NonNull::new(p as *mut u8);
        ptr.ok_or(AllocError::NoMemory)
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        // warn!("dealloc {}", layout.size());
        let addr = pos.as_ptr() as usize;
        assert!(addr >= self.start && addr < self.next);
        assert!(self.byte_used >= layout.size());
        self.byte_used -= layout.size();
        if self.byte_used == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        assert!(self.page_next >= self.start);
        self.page_next - self.start
    }

    fn used_bytes(&self) -> usize {
        self.byte_used
    }

    fn available_bytes(&self) -> usize {
        let total = self.total_bytes();
        let used = self.used_bytes();
        assert!(total >= used);
        total - used
    }
}

impl<const PAGE_SIZE:usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> allocator::AllocResult<usize> {
        if align_pow2 % Self::PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / Self::PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        let align_log2 = align_pow2.trailing_zeros() as usize;
        let bits = align_log2 + Self::PAGE_SIZE.trailing_zeros() as usize;
        let align_mask = (usize::MAX as usize) << bits;

        let page_next = self.page_next - num_pages * Self::PAGE_SIZE;
        let page_next = page_next & align_mask;

        if page_next < self.next {
            return Err(AllocError::NoMemory);
        }

        self.page_next = page_next;
        Ok(self.page_next)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        (self.end - self.next) / Self::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.page_next) / Self::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        self.total_pages() - self.used_pages()
    }
}
