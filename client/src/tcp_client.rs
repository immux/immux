use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::errors::ImmuxDBTcpClientResult;

use crate::ImmuxDBClient;
use immuxsys::server::tcp_response::TcpResponse;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::command::Command;
use immuxsys::storage::executor::command::SelectCondition;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::predicate::Predicate;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use immuxsys::storage::transaction_manager::TransactionId;

pub struct ImmuxDBTcpClient {
    stream: RefCell<TcpStream>,
}

fn write(stream: &mut TcpStream, buffer: &[u8]) -> ImmuxDBTcpClientResult<()> {
    stream.write_all(buffer)?;
    stream.flush()?;
    return Ok(());
}

fn read(stream: &mut TcpStream) -> ImmuxDBTcpClientResult<Vec<u8>> {
    let mut buffer = [0; 1024 * 5];
    stream.read(&mut buffer)?;
    return Ok(buffer.to_vec());
}

impl ImmuxDBTcpClient {
    pub fn new(host: &String) -> ImmuxDBTcpClientResult<ImmuxDBTcpClient> {
        let stream = TcpStream::connect(host)?;
        let stream_cell = RefCell::new(stream);
        return Ok(ImmuxDBTcpClient {
            stream: stream_cell,
        });
    }
}

impl ImmuxDBClient<ImmuxDBTcpClientResult<TcpResponse>> for ImmuxDBTcpClient {
    fn get_all_groupings(&self) -> ImmuxDBTcpClientResult<TcpResponse> {
        let condition = SelectCondition::AllGrouping;
        let command = Command::Select { condition };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn remove_groupings(&self, groupings: &[GroupingLabel]) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::RemoveGroupings {
            groupings: groupings.to_vec(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn get_by_key(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let condition = SelectCondition::Key(grouping.clone(), unit_key.clone(), None);
        let command = Command::Select { condition };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn get_by_predicate(
        &self,
        grouping: &GroupingLabel,
        predicate: &Predicate,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let condition = SelectCondition::Predicate(grouping.clone(), predicate.clone());
        let command = Command::Select { condition };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn transactional_get(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let condition = SelectCondition::Key(
            grouping.clone(),
            unit_key.clone(),
            Some(transaction_id.clone()),
        );
        let command = Command::Select { condition };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn inspect_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::InspectOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn inspect_all(&self) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::InspectAll;
        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn get_by_grouping(&self, grouping: &GroupingLabel) -> ImmuxDBTcpClientResult<TcpResponse> {
        let condition = SelectCondition::UnconditionalMatch(grouping.clone());
        let command = Command::Select { condition };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::Insert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn transactional_set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::TransactionalInsert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::RevertOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            height: height.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn transactional_revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::TransactionalRevertOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            height: height.clone(),
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn revert_all(&self, height: &ChainHeight) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::RevertAll {
            height: height.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn remove_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::RemoveOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn transactional_remove_one(
        &self,
        transaction_id: &TransactionId,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::TransactionalRemoveOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn remove_all(&self) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::RemoveAll;

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn create_transaction(&self) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::CreateTransaction;

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn commit_transaction(
        &self,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::TransactionCommit {
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }

    fn abort_transaction(
        &self,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<TcpResponse> {
        let command = Command::TransactionAbort {
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (response_str, _) = TcpResponse::parse(&buffer)?;

        return Ok(response_str);
    }
}
