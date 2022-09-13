use crate::GlobalMemory;

pub struct RawVec {
    ptr: *const u8,
    capacity: usize
}
