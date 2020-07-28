use std::fs::File;
use std::io::Result;
use std::io::{BufWriter, Seek, SeekFrom, Write};

pub struct LogWriter {
    writer: BufWriter<File>,
    pos: u64,
}

impl LogWriter {
    pub fn new(file: File) -> Result<Self> {
        let mut writer = BufWriter::new(file);
        let current_pos = writer.seek(SeekFrom::Current(0))?;

        Ok(LogWriter {
            writer,
            pos: current_pos,
        })
    }

    pub fn get_current_pos(&self) -> u64 {
        self.pos
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<()> {
        self.writer.write_all(data)?;
        self.pos += data.len() as u64;
        return Ok(());
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
