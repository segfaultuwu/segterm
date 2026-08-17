
use super::Allocator;

pub const HEAP_SIZE: usize = 16 * 1024 * 1024;

#[repr(align(4096))]
struct Heap {
    memory: [u8; HEAP_SIZE],
}

static mut HEAP: Heap = Heap {
    memory: [0; HEAP_SIZE],
};

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::new();

pub fn init_heap() {
    let start = unsafe { core::ptr::addr_of_mut!(HEAP.memory) as *mut u8 as usize };

    unsafe {
        ALLOCATOR.init(start, HEAP_SIZE);
    }
}
