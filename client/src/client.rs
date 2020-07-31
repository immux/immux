use crate::errors::{ClientResult, ImmuxDBClientError};

use immuxsys::constants as Constants;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use immuxsys::storage::transaction_manager::TransactionId;

use reqwest::Client;

pub struct ImmuxDBClient {
    host: String,
    client: Client,
}

impl ImmuxDBClient {
    pub fn new(host: &str) -> Result<ImmuxDBClient, ImmuxDBClientError> {
        return Ok(ImmuxDBClient {
            host: host.to_string(),
            client: reqwest::Client::new(),
        });
    }
}

impl ImmuxDBClient {
    pub fn get_by_key(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> ClientResult {
        let url = format!(
            "http://{}/{}/{}",
            &self.host,
            grouping.to_string(),
            unit_key.to_string()
        );
        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_get(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        transaction_id: &TransactionId,
    ) -> ClientResult {
        let url = format!(
            "http://{}/{}/{}/{}/{}",
            &self.host,
            Constants::URL_TRANSACTIONS_KEY_WORD,
            transaction_id.as_u64(),
            grouping.to_string(),
            unit_key.to_string(),
        );

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn inspect_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> ClientResult {
        let url = format!(
            "http://{}/{}/{}/{}",
            &self.host,
            grouping.to_string(),
            unit_key.to_string(),
            Constants::URL_JOURNAL_KEY_WORD,
        );

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn inspect_all(&self) -> ClientResult {
        let url = format!("http://{}/{}", &self.host, Constants::URL_JOURNAL_KEY_WORD,);

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn get_by_grouping(&self, _grouping: &GroupingLabel) -> ClientResult {
        return Err(ImmuxDBClientError::Unimplemented);
    }

    pub fn set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> ClientResult {
        let mut response = self
            .client
            .put(&format!(
                "http://{}/{}/{}",
                &self.host,
                grouping.to_string(),
                unit_key.to_string(),
            ))
            .body(unit_content.to_string())
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
        transaction_id: &TransactionId,
    ) -> ClientResult {
        let mut response = self
            .client
            .put(&format!(
                "http://{}/{}/{}/{}/{}",
                &self.host,
                Constants::URL_TRANSACTIONS_KEY_WORD,
                transaction_id.as_u64(),
                grouping.to_string(),
                unit_key.to_string(),
            ))
            .body(unit_content.to_string())
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
    ) -> ClientResult {
        let mut response = self
            .client
            .put(&format!(
                "http://{}/{}/{}?{}={}",
                &self.host,
                grouping.to_string(),
                unit_key.to_string(),
                Constants::HEIGHT,
                height.as_u64(),
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
        transaction_id: &TransactionId,
    ) -> ClientResult {
        let mut response = self
            .client
            .put(&format!(
                "http://{}/{}/{}/{}/{}?{}={}",
                &self.host,
                Constants::URL_TRANSACTIONS_KEY_WORD,
                transaction_id.as_u64(),
                grouping.to_string(),
                unit_key.to_string(),
                Constants::HEIGHT,
                height.as_u64(),
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn revert_all(&self, height: &ChainHeight) -> ClientResult {
        let mut response = self
            .client
            .put(&format!(
                "http://{}/?{}={}",
                &self.host,
                Constants::HEIGHT,
                height.as_u64(),
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn remove_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> ClientResult {
        let mut response = self
            .client
            .delete(&format!(
                "http://{}/{}/{}",
                &self.host,
                grouping.to_string(),
                unit_key.to_string(),
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_remove_one(
        &self,
        transaction_id: &TransactionId,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> ClientResult {
        let mut response = self
            .client
            .delete(&format!(
                "http://{}/{}/{}/{}/{}",
                &self.host,
                Constants::URL_TRANSACTIONS_KEY_WORD,
                transaction_id.as_u64(),
                grouping.to_string(),
                unit_key.to_string(),
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn remove_all(&self) -> ClientResult {
        let mut response = self
            .client
            .delete(&format!("http://{}/", &self.host))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn create_transaction(&self) -> ClientResult {
        let mut response = self
            .client
            .post(&format!(
                "http://{}/{}",
                &self.host,
                Constants::URL_TRANSACTIONS_KEY_WORD,
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn commit_transaction(&self, transaction_id: &TransactionId) -> ClientResult {
        let mut response = self
            .client
            .post(&format!(
                "http://{}/{}/{}?{}",
                &self.host,
                Constants::URL_TRANSACTIONS_KEY_WORD,
                transaction_id.as_u64(),
                Constants::COMMIT_TRANSACTION_KEY_WORD,
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn abort_transaction(&self, transaction_id: &TransactionId) -> ClientResult {
        let mut response = self
            .client
            .post(&format!(
                "http://{}/{}/{}?{}",
                &self.host,
                Constants::URL_TRANSACTIONS_KEY_WORD,
                transaction_id.as_u64(),
                Constants::ABORT_TRANSACTION_KEY_WORD,
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }
}
