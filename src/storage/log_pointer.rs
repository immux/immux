pub struct LogPointer {
    pub pos: u64,
    pub len: usize,
}

impl LogPointer {
    pub fn new(pos: u64, len: usize) -> LogPointer {
        LogPointer { pos, len }
    }
}
