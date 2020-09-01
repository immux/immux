use std::net::TcpStream;

use crate::errors::TCPClientResult;

use immuxsys::storage::executor::command::Command;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use std::io::{Read, Write};

pub struct ImmuxDBTCPClient {
    stream: TcpStream,
}

impl ImmuxDBTCPClient {
    pub fn new(host: &str) -> TCPClientResult<ImmuxDBTCPClient> {
        let stream = TcpStream::connect(host)?;
        return Ok(ImmuxDBTCPClient { stream });
    }

    pub fn set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> TCPClientResult<()> {
        let mut stream = &self.stream;
        let command = Command::Insert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
        };

        let command_bytes = command.marshal();

        stream.write_all(&command_bytes)?;

        return Ok(());
    }
}
