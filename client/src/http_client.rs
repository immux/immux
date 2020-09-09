use crate::errors::{HttpClientResult, ImmuxDBHttpClientError};

use immuxsys::constants as Constants;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::filter::Filter;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use immuxsys::storage::transaction_manager::TransactionId;

use reqwest::Client;
use reqwest::Url;

pub struct ImmuxDBHttpClient {
    host: String,
    client: Client,
}

impl ImmuxDBHttpClient {
    pub fn new(host: &str) -> Result<ImmuxDBHttpClient, ImmuxDBHttpClientError> {
        return Ok(ImmuxDBHttpClient {
            host: host.to_string(),
            client: reqwest::Client::new(),
        });
    }

    pub fn get_by_key(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn get_by_filter(&self, grouping: &GroupingLabel, filter: &Filter) -> HttpClientResult {
        let url_str = format!("http://{}/{}", &self.host, grouping.to_string());

        let url = Url::parse_with_params(&url_str, &[("filter", format!("{}", filter))]).unwrap();

        let mut response = self.client.get(url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_get(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        transaction_id: &TransactionId,
    ) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn inspect_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn inspect_all(&self) -> HttpClientResult {
        let url = format!("http://{}/{}", &self.host, Constants::URL_JOURNAL_KEY_WORD,);

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn get_by_grouping(&self, grouping: &GroupingLabel) -> HttpClientResult {
        let url = format!("http://{}/{}", &self.host, grouping);

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBClientError::Reqwest(error.into())),
        }
    }

    pub fn set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
        transaction_id: &TransactionId,
    ) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
    ) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
        transaction_id: &TransactionId,
    ) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn revert_all(&self, height: &ChainHeight) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn remove_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn transactional_remove_one(
        &self,
        transaction_id: &TransactionId,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn remove_all(&self) -> HttpClientResult {
        let mut response = self
            .client
            .delete(&format!("http://{}/", &self.host))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn create_transaction(&self) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn commit_transaction(&self, transaction_id: &TransactionId) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    pub fn abort_transaction(&self, transaction_id: &TransactionId) -> HttpClientResult {
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
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }
}
