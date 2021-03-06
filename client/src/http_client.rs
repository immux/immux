use reqwest::Client;
use reqwest::Url;

use crate::errors::{HttpClientResult, ImmuxDBHttpClientError};
use crate::ImmuxDBClient;

use immuxsys::constants as Constants;
use immuxsys::constants::PREDICATE_URL_KEY;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::predicate::Predicate;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use immuxsys::storage::transaction_manager::TransactionId;

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
}

impl ImmuxDBClient<HttpClientResult> for ImmuxDBHttpClient {
    fn get_all_groupings(&self) -> HttpClientResult {
        let url = format!("http://{}/{}", &self.host, Constants::URL_GROUPING_KEY_WORD,);

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn remove_groupings(&self, groupings: &[GroupingLabel]) -> HttpClientResult {
        let url = format!("http://{}/{}", &self.host, Constants::URL_GROUPING_KEY_WORD,);
        let grouping_names = groupings
            .iter()
            .map(|grouping| format!("{}", &grouping))
            .collect::<Vec<String>>()
            .join("\r\n");

        let mut response = self.client.delete(&url).body(grouping_names).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn get_by_key(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> HttpClientResult {
        let url = format!(
            "http://{}/{}/{}",
            &self.host,
            format!("{}", grouping),
            format!("{}", unit_key),
        );
        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn get_by_predicate(
        &self,
        grouping: &GroupingLabel,
        predicate: &Predicate,
    ) -> HttpClientResult {
        let url_str = format!("http://{}/{}", &self.host, grouping);

        let url =
            Url::parse_with_params(&url_str, &[(PREDICATE_URL_KEY, format!("{}", predicate))])
                .unwrap();

        let mut response = self.client.get(url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn transactional_get(
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
            grouping,
            unit_key,
        );

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn inspect_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> HttpClientResult {
        let url = format!(
            "http://{}/{}/{}/{}",
            &self.host,
            grouping,
            unit_key,
            Constants::URL_JOURNAL_KEY_WORD,
        );

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn inspect_all(&self) -> HttpClientResult {
        let url = format!("http://{}/{}", &self.host, Constants::URL_JOURNAL_KEY_WORD,);

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn get_by_grouping(&self, grouping: &GroupingLabel) -> HttpClientResult {
        let url = format!("http://{}/{}", &self.host, grouping);

        let mut response = self.client.get(&url).send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> HttpClientResult {
        let mut response = self
            .client
            .put(&format!("http://{}/{}/{}", &self.host, grouping, unit_key,))
            .body(format!("{}", unit_content))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn transactional_set_unit(
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
                grouping,
                unit_key,
            ))
            .body(format!("{}", unit_content))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn revert_one(
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
                grouping,
                unit_key,
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

    fn transactional_revert_one(
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
                grouping,
                unit_key,
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

    fn revert_all(&self, height: &ChainHeight) -> HttpClientResult {
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

    fn remove_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> HttpClientResult {
        let mut response = self
            .client
            .delete(&format!("http://{}/{}/{}", &self.host, grouping, unit_key,))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn transactional_remove_one(
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
                grouping,
                unit_key,
            ))
            .send()?;
        let status_code = response.status();

        match response.text() {
            Ok(text) => Ok((status_code, text)),
            Err(error) => Err(ImmuxDBHttpClientError::Reqwest(error.into())),
        }
    }

    fn remove_all(&self) -> HttpClientResult {
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

    fn create_transaction(&self) -> HttpClientResult {
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

    fn commit_transaction(&self, transaction_id: &TransactionId) -> HttpClientResult {
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

    fn abort_transaction(&self, transaction_id: &TransactionId) -> HttpClientResult {
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
