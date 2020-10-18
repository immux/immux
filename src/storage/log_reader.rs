use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Result, Seek, SeekFrom};
use std::path::PathBuf;

use crate::storage::errors::KVResult;
use crate::storage::instruction::{unpack_instruction, Instruction};
use crate::storage::instruction_iterator::InstructionLogIterator;
use crate::storage::log_pointer::LogPointer;
use crate::storage::log_version::LogVersion;

pub struct LogReader {
    buf_reader: BufReader<File>,
    log_version: LogVersion,
}

impl LogReader {
    pub fn new(file_path: &PathBuf, log_version: LogVersion) -> Result<Self> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let reader = BufReader::new(file);

        Ok(LogReader {
            buf_reader: reader,
            log_version,
        })
    }

    pub fn read_pointer(&mut self, log_pointer: &LogPointer) -> KVResult<Instruction> {
        let mut buffer = vec![0; log_pointer.len];
        self.buf_reader.seek(SeekFrom::Start(log_pointer.pos))?;
        self.buf_reader.read_exact(&mut buffer)?;
        let (instruction, _) = unpack_instruction(&buffer)?;
        return Ok(instruction);
    }

    pub fn read_all(&mut self) -> KVResult<(InstructionLogIterator, usize)> {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.buf_reader.capacity());
        self.buf_reader.seek(SeekFrom::Start(0))?;
        self.buf_reader.read_to_end(&mut buffer)?;

        let (target_log_version, offset) = LogVersion::parse(&buffer)?;

        assert!(self.log_version >= target_log_version);

        let iterator = InstructionLogIterator::from(buffer[offset..].to_vec());
        return Ok((iterator, offset));
    }
}
