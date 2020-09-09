use std::net::TcpStream;

use crate::errors::ImmuxDBTcpClientResult;
use std::io::{Read, Write};

pub struct ImmuxDBTcpClient {
    stream: TcpStream,
}

impl ImmuxDBTcpClient {
    pub fn new(host: &String) -> ImmuxDBTcpClientResult<ImmuxDBTcpClient> {
        let stream = TcpStream::connect(host)?;
        return Ok(ImmuxDBTcpClient { stream });
    }

    pub fn write(&mut self, buffer: &[u8]) -> ImmuxDBTcpClientResult<()> {
        self.stream.write_all(buffer)?;
        self.stream.flush()?;
        return Ok(());
    }

    pub fn read(&mut self) -> ImmuxDBTcpClientResult<Vec<u8>> {
        let mut buffer = [0; 1024 * 5];
        self.stream.read(&mut buffer)?;
        return Ok(buffer.to_vec());
    }
}
