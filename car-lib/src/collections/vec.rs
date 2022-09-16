use crate::GlobalMemory;

pub struct RawVec<'a> {
    global_memory: &'a GlobalMemory,
    ptr: *const u8,
    len: usize,
    capacity: usize
}

impl<'a> RawVec<'a> {
    fn new(global_memory: &'a mut GlobalMemory) -> Self {
        let ptr = global_memory.allocate(4usize);
        Self {
            global_memory,
            ptr,
            len: 0usize,
            capacity: 4usize,
        }
    }
}
