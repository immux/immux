use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::filter::Filter;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use immuxsys::storage::transaction_manager::TransactionId;

pub mod errors;
pub mod http_client;
pub mod tcp_client;

pub trait ImmuxDBClient<T> {
    fn get_by_key(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> T;

    fn get_by_filter(&self, grouping: &GroupingLabel, filter: &Filter) -> T;

    fn transactional_get(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        transaction_id: &TransactionId,
    ) -> T;

    fn inspect_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> T;

    fn inspect_all(&self) -> T;

    fn get_by_grouping(&self, grouping: &GroupingLabel) -> T;

    fn set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
    ) -> T;

    fn transactional_set_unit(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        unit_content: &UnitContent,
        transaction_id: &TransactionId,
    ) -> T;

    fn revert_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey, height: &ChainHeight) -> T;

    fn transactional_revert_one(
        &self,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
        height: &ChainHeight,
        transaction_id: &TransactionId,
    ) -> T;

    fn revert_all(&self, height: &ChainHeight) -> T;

    fn remove_one(&self, grouping: &GroupingLabel, unit_key: &UnitKey) -> T;

    fn transactional_remove_one(
        &self,
        transaction_id: &TransactionId,
        grouping: &GroupingLabel,
        unit_key: &UnitKey,
    ) -> T;

    fn remove_all(&self) -> T;

    fn create_transaction(&self) -> T;

    fn commit_transaction(&self, transaction_id: &TransactionId) -> T;

    fn abort_transaction(&self, transaction_id: &TransactionId) -> T;

    fn get_all_groupings(&self) -> T;
}
