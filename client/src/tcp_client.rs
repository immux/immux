use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::errors::ImmuxDBTcpClientResult;

use crate::ImmuxDBClient;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::command::Command;
use immuxsys::storage::executor::command::SelectCondition;
use immuxsys::storage::executor::filter::Filter;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::outcome::Outcome;
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

impl ImmuxDBClient<ImmuxDBTcpClientResult<Outcome>> for ImmuxDBTcpClient {
    fn get_by_key(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let condition = SelectCondition::Key(unit_key.clone(), None);
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn get_by_filter(
        &self,
        grouping: &GroupingLabel,
        filter: &Filter,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let condition = SelectCondition::Filter(filter.clone());
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn transactional_get(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let condition = SelectCondition::Key(unit_key.clone(), Some(transaction_id.clone()));
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn inspect_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::InspectOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn inspect_all(&self) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::InspectAll;
        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn get_by_grouping(&self, grouping: &GroupingLabel) -> ImmuxDBTcpClientResult<Outcome> {
        let condition = SelectCondition::UnconditionalMatch;
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;
        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::Insert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn transactional_set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionalInsert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RevertOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            height: height.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn transactional_revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionalRevertOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            height: height.clone(),
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn revert_all(&self, height: &ChainHeight) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RevertAll {
            height: height.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn remove_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RemoveOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn transactional_remove_one(
        &self,
        transaction_id: &TransactionId,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionalRemoveOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn remove_all(&self) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RemoveAll;

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn create_transaction(&self) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::CreateTransaction;

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn commit_transaction(
        &self,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionCommit {
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    fn abort_transaction(&self, transaction_id: &TransactionId) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionAbort {
            transaction_id: transaction_id.clone(),
        };

        let mut stream = self.stream.borrow_mut();
        write(&mut *stream, &command.marshal())?;

        let buffer = read(&mut *stream)?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }
}
