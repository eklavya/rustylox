use std::alloc::Layout;

pub struct GC {}

impl GC {
    pub fn alloc(layout: Layout) -> *mut u8 {
        unsafe { std::alloc::alloc(layout) }
    }

    pub fn dealloc(ptr: *mut u8, layout: Layout) {
        unsafe {
            std::alloc::dealloc(ptr, layout);
        }
    }
}
