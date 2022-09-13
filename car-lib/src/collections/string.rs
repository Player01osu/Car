use crate::GlobalMemory;

pub struct String {
    ptr: *const u8,
    length: usize,
    capacity: usize,
}

impl String {
    pub fn new(global_mem: &mut GlobalMemory) -> Self {
        let content = global_mem.allocate(4usize);
        Self {
            ptr: content,
            length: 0,
            capacity: 4,
        }
    }
}

