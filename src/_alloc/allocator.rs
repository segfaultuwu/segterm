use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Allocator {
    start: AtomicUsize,
    end: AtomicUsize,
    next: AtomicUsize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            start: AtomicUsize::new(0),
            end: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
        }
    }

    pub unsafe fn init(&self, start: usize, size: usize) {
        self.start.store(start, Ordering::SeqCst);
        self.end.store(start + size, Ordering::SeqCst);
        self.next.store(start, Ordering::SeqCst);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        if size == 0 {
            return null_mut();
        }

        let mut current = self.next.load(Ordering::Relaxed);

        loop {
            let aligned = match current.checked_add(align - 1) {
                Some(value) => value & !(align - 1),
                None => return null_mut(),
            };

            let end = match aligned.checked_add(size) {
                Some(value) => value,
                None => return null_mut(),
            };

            if end > self.end.load(Ordering::Acquire) {
                return null_mut();
            }

            match self
                .next
                .compare_exchange_weak(current, end, Ordering::AcqRel, Ordering::Relaxed)
            {
                Ok(_) => return aligned as *mut u8,
                Err(actual) => current = actual,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Does not actually deallocate memory lol
    }
}
