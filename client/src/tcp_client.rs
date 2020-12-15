use std::io::{Read, Write};
use std::net::TcpStream;

use crate::errors::ImmuxDBTcpClientResult;

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
    stream: TcpStream,
}

impl ImmuxDBTcpClient {
    pub fn new(host: &String) -> ImmuxDBTcpClientResult<ImmuxDBTcpClient> {
        let stream = TcpStream::connect(host)?;
        return Ok(ImmuxDBTcpClient { stream });
    }

    pub fn get_by_key(
        &mut self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let condition = SelectCondition::Key(unit_key.clone(), None);
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        self.write(&command.marshal())?;
        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn get_by_filter(
        &mut self,
        grouping: &GroupingLabel,
        filter: &Filter,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let condition = SelectCondition::Filter(filter.clone());
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn transactional_get(
        &mut self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let condition = SelectCondition::Key(unit_key.clone(), Some(transaction_id.clone()));
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn inspect_one(
        &mut self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::InspectOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn set_unit(
        &mut self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::Insert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn transactional_set_unit(
        &mut self,
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

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn revert_one(
        &mut self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RevertOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            height: height.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn transactional_revert_one(
        &mut self,
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

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn revert_all(&mut self, height: &ChainHeight) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RevertAll {
            height: height.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn remove_one(
        &mut self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RemoveOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn transactional_remove_one(
        &mut self,
        transaction_id: &TransactionId,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionalRemoveOne {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            transaction_id: transaction_id.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn remove_all(&mut self) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::RemoveAll;

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn create_transaction(&mut self) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::CreateTransaction;

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn commit_transaction(
        &mut self,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionCommit {
            transaction_id: transaction_id.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
    }

    pub fn abort_transaction(
        &mut self,
        transaction_id: &TransactionId,
    ) -> ImmuxDBTcpClientResult<Outcome> {
        let command = Command::TransactionAbort {
            transaction_id: transaction_id.clone(),
        };

        self.write(&command.marshal())?;

        let buffer = self.read()?;
        let (outcome, _) = Outcome::parse(&buffer)?;

        return Ok(outcome);
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
