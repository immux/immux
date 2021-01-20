#[derive(Debug, Clone)]
pub struct LogPointer {
    /// The starting position of the instruction in the log file.
    pub pos: u64,

    /// The length of the instruction.
    pub len: usize,
}

impl LogPointer {
    pub fn new(pos: u64, len: usize) -> LogPointer {
        LogPointer { pos, len }
    }
}
