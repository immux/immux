#[cfg(test)]
mod kv_tests {

    use std::collections::HashMap;
    use std::fs::remove_file;
    use std::path::PathBuf;

    use immuxsys::storage::chain_height::ChainHeight;
    use immuxsys::storage::executor::command::SelectCondition;
    use immuxsys::storage::executor::executor::Executor;
    use immuxsys::storage::executor::grouping_label::GroupingLabel;
    use immuxsys::storage::executor::outcome::Outcome;
    use immuxsys::storage::executor::predicate::{
        CompoundPredicate, FieldPath, Predicate, PrimitivePredicate,
    };
    use immuxsys::storage::executor::unit_content::UnitContent;
    use immuxsys::storage::executor::unit_key::UnitKey;
    use immuxsys::storage::kv::{get_main_log_full_path, LogKeyValueStore};
    use immuxsys::storage::kvkey::KVKey;
    use immuxsys::storage::kvvalue::KVValue;
    use immuxsys::storage::preferences::DBPreferences;
    use immuxsys::storage::transaction_manager::TransactionId;
    use immuxsys::utils::ints::{u64_to_u8_array, u8_array_to_u64};

    fn reset_dir_path(log_dir_str: &str) {
        let test_dir_path = PathBuf::from(log_dir_str);
        let log_file_path = get_main_log_full_path(&test_dir_path);

        if log_file_path.exists() {
            remove_file(log_file_path).unwrap();
        }
    }

    fn get_store_engine(test_data_dir: &str) -> LogKeyValueStore {
        reset_dir_path(test_data_dir);

        let pref = DBPreferences::default_at_dir(test_data_dir);

        let store_engine = LogKeyValueStore::open(&pref).unwrap();

        return store_engine;
    }

    fn get_test_predicate() -> Predicate {
        // this.age >= 12 && this.height <= 170 && this.name == "Bob" && this.boy == true
        return Predicate::Compound(CompoundPredicate::And(vec![
            Predicate::Primitive(PrimitivePredicate::GreaterThan(
                FieldPath::from(vec![String::from("age")]),
                UnitContent::Float64(12.0),
            )),
            Predicate::Primitive(PrimitivePredicate::LessThan(
                FieldPath::from(vec![String::from("height")]),
                UnitContent::Float64(170.0),
            )),
            Predicate::Primitive(PrimitivePredicate::Equal(
                FieldPath::from(vec![String::from("name")]),
                UnitContent::String(String::from("Bob")),
            )),
            Predicate::Primitive(PrimitivePredicate::Equal(
                FieldPath::from(vec![String::from("boy")]),
                UnitContent::Bool(true),
            )),
        ]));
    }

    #[test]
    fn kv_open() {
        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let expected_value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);
        let data_dir = "/tmp/kv_open";
        {
            let mut store_engine = get_store_engine(data_dir);
            store_engine.set(&key, &expected_value, None).unwrap();
            let actual_value = store_engine.get(&key, None).unwrap().unwrap();

            assert_eq!(actual_value, expected_value);
        }

        {
            let pref = DBPreferences::default_at_dir(data_dir);
            let mut store_engine = LogKeyValueStore::open(&pref).unwrap();
            let actual_value = store_engine.get(&key, None).unwrap().unwrap();

            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn incomplete_transaction_manager() {
        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);
        let tid_snapshot;
        let data_dir = "/tmp/incomplete_transaction_manager";
        {
            let mut store_engine = get_store_engine(data_dir);
            let tid = store_engine.start_transaction().unwrap();
            tid_snapshot = tid.clone();
            store_engine.set(&key, &value, Some(tid)).unwrap();
            let output = store_engine.get(&key, Some(tid)).unwrap().unwrap();
            assert_eq!(output, value);
        }

        {
            let pref = DBPreferences::default_at_dir(data_dir);
            let mut store_engine = LogKeyValueStore::open(&pref).unwrap();
            let actual_value = store_engine.get(&key, None).unwrap();
            assert!(actual_value.is_none());

            let result = store_engine.commit_transaction(tid_snapshot);
            assert!(result.is_err());
        }
    }

    #[test]
    fn kv_set() {
        let mut store_engine = get_store_engine("/tmp/test_set");

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let expected_value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);

        store_engine.set(&key, &expected_value, None).unwrap();
        let actual_value = store_engine.get(&key, None).unwrap().unwrap();

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn kv_revert_one() {
        let mut store_engine = get_store_engine("/tmp/test_revert_one");
        let target_height = ChainHeight::new(2);

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let values: Vec<KVValue> = vec![
            KVValue::new(&[0x00]),
            KVValue::new(&[0x01]),
            KVValue::new(&[0x02]),
            KVValue::new(&[0x03]),
            KVValue::new(&[0x04]),
            KVValue::new(&[0x05]),
        ];

        for value in &values {
            store_engine.set(&key, &value, None).unwrap();
        }

        store_engine.revert_one(&key, &target_height, None).unwrap();

        let actual_output = store_engine.get(&key, None).unwrap().unwrap();

        let expected_output = &values.as_slice()[target_height.as_u64() as usize];

        assert_eq!(&actual_output, expected_output);
    }

    #[test]
    fn kv_revert_all() {
        let mut store_engine = get_store_engine("/tmp/test_revert_all");
        let target_height = ChainHeight::new(5);

        let key_value_pairs = vec![
            (KVKey::new(&[0x00]), KVValue::new(&[0x00])),
            (KVKey::new(&[0x00]), KVValue::new(&[0xff])),
            (KVKey::new(&[0x00]), KVValue::new(&[0x22])),
            (KVKey::new(&[0x01]), KVValue::new(&[0x01])),
            (KVKey::new(&[0x00]), KVValue::new(&[0x19])),
            (KVKey::new(&[0x02]), KVValue::new(&[0x02])),
            (KVKey::new(&[0x03]), KVValue::new(&[0x03])),
            (KVKey::new(&[0x04]), KVValue::new(&[0x04])),
            (KVKey::new(&[0x05]), KVValue::new(&[0x05])),
        ];

        for kv_pair in &key_value_pairs {
            store_engine.set(&kv_pair.0, &kv_pair.1, None).unwrap();
        }

        store_engine.revert_all(&target_height).unwrap();

        let mut expected_hashmap = HashMap::new();

        for expected_kv_pair in &key_value_pairs[..target_height.as_u64() as usize] {
            expected_hashmap.insert(&expected_kv_pair.0, &expected_kv_pair.1);
        }

        for (key, expected_value) in expected_hashmap {
            let actual_value = store_engine.get(&key, None).unwrap().unwrap();
            assert_eq!(expected_value, &actual_value);
        }
    }

    #[test]
    fn kv_remove() {
        let mut store_engine = get_store_engine("/tmp/test_remove");

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let expected_value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);

        store_engine.set(&key, &expected_value, None).unwrap();
        store_engine.remove_one(&key, None).unwrap();

        let actual_value = store_engine.get(&key, None).unwrap();

        assert_eq!(actual_value, None);
    }

    #[test]
    fn kv_remove_all() {
        let mut store_engine = get_store_engine("/tmp/test_remove_all");

        let key_value_pairs = vec![
            (KVKey::new(&[0xff]), KVValue::new(&[0x00])),
            (KVKey::new(&[0xf2]), KVValue::new(&[0xff])),
            (KVKey::new(&[0x23]), KVValue::new(&[0x22])),
            (KVKey::new(&[0x11]), KVValue::new(&[0x01])),
        ];

        for kv_pair in &key_value_pairs {
            store_engine.set(&kv_pair.0, &kv_pair.1, None).unwrap();
        }

        store_engine.remove_all().unwrap();

        for kv_pair in &key_value_pairs {
            let out_put = store_engine.get(&kv_pair.0, None).unwrap();
            assert_eq!(out_put, None);
        }
    }

    #[test]
    fn test_content_satisfied_filter_unit() {
        let satisfied_contents = vec![{
            let mut map = HashMap::new();
            map.insert(String::from("age"), UnitContent::Float64(14.0));
            map.insert(String::from("height"), UnitContent::Float64(168.0));
            map.insert(
                String::from("name"),
                UnitContent::String(String::from("Bob")),
            );
            map.insert(String::from("boy"), UnitContent::Bool(true));
            UnitContent::Map(map)
        }];

        let unsatisfied_contents = vec![
            {
                let mut map = HashMap::new();
                map.insert(String::from("age"), UnitContent::Float64(12.0));
                map.insert(String::from("height"), UnitContent::Float64(170.0));
                map.insert(
                    String::from("name"),
                    UnitContent::String(String::from("Jerry")),
                );
                map.insert(String::from("boy"), UnitContent::Bool(false));
                UnitContent::Map(map)
            },
            {
                let mut map = HashMap::new();
                map.insert(String::from("age"), UnitContent::Float64(20.0));
                map.insert(String::from("height"), UnitContent::Float64(181.0));
                map.insert(
                    String::from("name"),
                    UnitContent::String(String::from("Tom")),
                );
                map.insert(String::from("boy"), UnitContent::Bool(false));
                UnitContent::Map(map)
            },
            {
                let mut map = HashMap::new();
                map.insert(String::from("age"), UnitContent::Float64(2.0));
                map.insert(String::from("height"), UnitContent::Float64(157.0));
                map.insert(
                    String::from("name"),
                    UnitContent::String(String::from("Andy")),
                );
                map.insert(String::from("boy"), UnitContent::Bool(true));
                UnitContent::Map(map)
            },
        ];
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let predicate = get_test_predicate();
        let condition = SelectCondition::Predicate(grouping.clone(), predicate);

        let mut contents = vec![];
        contents.extend_from_slice(&satisfied_contents);
        contents.extend_from_slice(&unsatisfied_contents);

        let log_dir_str = "/tmp/test_content_satisfied_filter_unit";

        reset_dir_path(log_dir_str);

        let pref = DBPreferences::default_at_dir("/tmp/test_content_satisfied_filter_unit");

        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let mut executor = Executor::open(&pref).unwrap();

        for (index, content) in contents.iter().enumerate() {
            let unit_key = UnitKey::from(format!("{}", index).as_str());
            executor.set(&grouping, &unit_key, &content, None).unwrap();
        }

        let outcome = executor.get(&condition).unwrap();

        match outcome {
            Outcome::Select(output) => {
                for content in output.iter() {
                    assert!(&satisfied_contents.contains(content));
                }

                for content in satisfied_contents.iter() {
                    assert!(&output.contains(content));
                }

                assert_eq!(output.len(), satisfied_contents.len());
            }
            _ => panic!("Unexpected outcome"),
        }
    }

    #[test]
    fn kv_atomicity_commit() {
        let mut store_engine = get_store_engine("/tmp/test_atomicity_commit");

        let key_value_paris = [
            (KVKey::from("a"), KVValue::from("1")),
            (KVKey::from("b"), KVValue::from("2")),
            (KVKey::from("c"), KVValue::from("3")),
            (KVKey::from("d"), KVValue::from("4")),
        ];

        let transaction_id = store_engine.start_transaction().unwrap();
        for pair in &key_value_paris {
            store_engine
                .set(&pair.0, &pair.1, Some(transaction_id))
                .unwrap();
        }
        store_engine.commit_transaction(transaction_id).unwrap();

        for pair in &key_value_paris {
            let actual_value = store_engine.get(&pair.0, None).unwrap().unwrap();
            let expected_value = pair.1.clone();

            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn kv_atomicity_abort() {
        let mut store_engine = get_store_engine("/tmp/test_atomicity_abort");

        let key_value_paris = [
            (KVKey::from("1"), KVValue::from("a")),
            (KVKey::from("2"), KVValue::from("b")),
            (KVKey::from("3"), KVValue::from("c")),
        ];

        let expected_value = None;

        let mut transaction_id = store_engine.start_transaction().unwrap();
        for pair in &key_value_paris {
            store_engine
                .set(&pair.0, &pair.1, Some(transaction_id))
                .unwrap();
        }
        store_engine.abort_transaction(&mut transaction_id).unwrap();

        for pair in &key_value_paris {
            let actual_value = store_engine.get(&pair.0, None).unwrap();
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn kv_set_isolation() {
        let mut store_engine = get_store_engine("/tmp/test_set_isolation");

        let key = KVKey::from("a");
        let value = KVValue::from("1");

        let transaction_id = store_engine.start_transaction().unwrap();
        store_engine
            .set(&key, &value, Some(transaction_id))
            .unwrap();

        {
            let actual_value_within_transaction = store_engine
                .get(&key, Some(transaction_id))
                .unwrap()
                .unwrap();
            let expected_value_within_transaction = value.clone();

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        {
            let actual_value_outside_transaction = store_engine.get(&key, None).unwrap();
            let expected_value_outside_transaction = None;

            assert_eq!(
                expected_value_outside_transaction,
                actual_value_outside_transaction
            );
        }

        store_engine.commit_transaction(transaction_id).unwrap();

        {
            let actual_value_after_commit = store_engine.get(&key, None).unwrap().unwrap();
            let expected_value_after_commit = value.clone();

            assert_eq!(expected_value_after_commit, actual_value_after_commit);
        }
    }

    #[test]
    fn kv_remove_one_isolation() {
        let mut store_engine = get_store_engine("/tmp/test_remove_one_isolation");

        let key_value_paris = [
            (KVKey::from("one"), KVValue::from("a")),
            (KVKey::from("two"), KVValue::from("b")),
            (KVKey::from("three"), KVValue::from("c")),
        ];

        let kv_to_be_removed = &key_value_paris[0];

        for pair in &key_value_paris {
            store_engine
                .set(&pair.0.clone(), &pair.1.clone(), None)
                .unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();
        store_engine
            .remove_one(&kv_to_be_removed.0, Some(transaction_id))
            .unwrap();

        {
            let actual_value_within_transaction = store_engine
                .get(&kv_to_be_removed.0.clone(), Some(transaction_id))
                .unwrap();
            let expected_value_within_transaction = None;

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        {
            let actual_value_outside_transaction = store_engine
                .get(&&kv_to_be_removed.0.clone(), None)
                .unwrap()
                .unwrap();
            let expected_value_outside_transaction = kv_to_be_removed.1.clone();

            assert_eq!(
                actual_value_outside_transaction,
                expected_value_outside_transaction
            );
        }

        store_engine.commit_transaction(transaction_id).unwrap();

        {
            let actual_value_after_commit =
                store_engine.get(&kv_to_be_removed.0.clone(), None).unwrap();
            let expected_value_after_commit = None;

            assert_eq!(expected_value_after_commit, actual_value_after_commit);
        }
    }

    #[test]
    fn kv_revert_one_isolation() {
        let mut store_engine = get_store_engine("/tmp/test_revert_one_isolation");

        let key_value_paris = [
            (KVKey::from("one"), KVValue::from("a")),
            (KVKey::from("one"), KVValue::from("b")),
            (KVKey::from("one"), KVValue::from("c")),
            (KVKey::from("one"), KVValue::from("d")),
            (KVKey::from("one"), KVValue::from("e")),
        ];

        let index = 2;
        let target_height = ChainHeight::new(index);

        let kv_to_be_reverted = &key_value_paris[index as usize];

        for pair in &key_value_paris {
            store_engine.set(&pair.0, &pair.1, None).unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();
        store_engine
            .revert_one(&kv_to_be_reverted.0, &target_height, Some(transaction_id))
            .unwrap();

        {
            let actual_value_within_transaction = store_engine
                .get(&kv_to_be_reverted.0.clone(), Some(transaction_id))
                .unwrap()
                .unwrap();
            let expected_value_within_transaction = kv_to_be_reverted.1.clone();

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        {
            let actual_value_outside_transaction = store_engine
                .get(&kv_to_be_reverted.0.clone(), None)
                .unwrap()
                .unwrap();
            let expected_value_outside_transaction = key_value_paris.last().unwrap().1.clone();

            assert_eq!(
                actual_value_outside_transaction,
                expected_value_outside_transaction
            );
        }

        store_engine.commit_transaction(transaction_id).unwrap();

        {
            let actual_value_after_commit = store_engine
                .get(&kv_to_be_reverted.0.clone(), None)
                .unwrap()
                .unwrap();
            let expected_value_after_commit = kv_to_be_reverted.1.clone();

            assert_eq!(expected_value_after_commit, actual_value_after_commit);
        }
    }

    #[test]
    fn transaction_not_alive_after_remove_all() {
        let mut store_engine = get_store_engine("/tmp/transaction_not_alive_after_remove_all");

        let key_value_paris = [
            (KVKey::from("k1"), KVValue::from("a")),
            (KVKey::from("k2"), KVValue::from("b")),
            (KVKey::from("k3"), KVValue::from("c")),
        ];

        let target_kv = (KVKey::from("k4"), KVValue::from("d"));

        for pair in &key_value_paris {
            store_engine.set(&pair.0, &pair.1, None).unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();

        {
            store_engine
                .set(&target_kv.0, &target_kv.1, Some(transaction_id))
                .unwrap();
            let actual_value_within_transaction = &store_engine
                .get(&target_kv.0, Some(transaction_id))
                .unwrap()
                .unwrap();
            let expected_value_within_transaction = &target_kv.1;

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        store_engine.remove_all().unwrap();

        {
            for pair in &key_value_paris {
                let actual_value_outside_transaction =
                    store_engine.get(&pair.0.clone(), None).unwrap();
                let expected_value_outside_transaction = None;

                assert_eq!(
                    actual_value_outside_transaction,
                    expected_value_outside_transaction
                );
            }
        }

        assert!(store_engine.commit_transaction(transaction_id).is_err());
    }

    #[test]
    fn transaction_not_alive_after_revert_all() {
        let mut store_engine = get_store_engine("/tmp/test_revert_all_transaction_not_alive");

        let key_value_pairs = [
            (KVKey::from("1"), KVValue::from("a")),
            (KVKey::from("1"), KVValue::from("b")),
            (KVKey::from("1"), KVValue::from("c")),
            (KVKey::from("1"), KVValue::from("d")),
            (KVKey::from("1"), KVValue::from("e")),
            (KVKey::from("2"), KVValue::from("f")),
            (KVKey::from("3"), KVValue::from("g")),
            (KVKey::from("1"), KVValue::from("h")),
        ];

        let index = 6;
        let target_height = ChainHeight::new(index);

        for pair in &key_value_pairs {
            store_engine.set(&pair.0, &pair.1, None).unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();

        store_engine.revert_all(&target_height).unwrap();

        assert!(store_engine.commit_transaction(transaction_id).is_err());
    }

    #[test]
    #[should_panic]
    fn unexpected_commit_transaction_id() {
        let mut store_engine = get_store_engine("/tmp/test_unexpected_commit_transaction_id");

        let some_random_transaction_id = 100;

        store_engine
            .commit_transaction(TransactionId::new(some_random_transaction_id))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn unexpected_abort_transaction_id() {
        let mut store_engine = get_store_engine("/tmp/test_unexpected_abort_transaction_id");

        let some_random_transaction_id = 10;

        store_engine
            .abort_transaction(&mut TransactionId::new(some_random_transaction_id))
            .unwrap();
    }

    #[test]
    fn last_one_commit_wins() {
        let mut store_engine = get_store_engine("/tmp/test_last_one_commit_wins");

        let shared_keys = [KVKey::from("a"), KVKey::from("b"), KVKey::from("c")];

        let key_value_pairs_1: Vec<(KVKey, KVValue)> = shared_keys
            .iter()
            .enumerate()
            .map(|(index, key)| {
                let value = format!("{:?}", index + 1);
                (key.clone(), KVValue::from(value.as_str()))
            })
            .collect();

        let key_value_pairs_2: Vec<(KVKey, KVValue)> = shared_keys
            .iter()
            .enumerate()
            .map(|(index, key)| {
                let value = format!("{:?}", (index + 1) * 100);
                (key.clone(), KVValue::from(value.as_str()))
            })
            .collect();

        let mixed_kv_pairs: Vec<_> = key_value_pairs_1.iter().zip(&key_value_pairs_2).collect();

        let transaction_id_1 = store_engine.start_transaction().unwrap();
        let transaction_id_2 = store_engine.start_transaction().unwrap();

        for kv_pairs in mixed_kv_pairs {
            let kv_1 = kv_pairs.0;
            let kv_2 = kv_pairs.1;
            store_engine
                .set(&kv_1.0, &kv_1.1, Some(transaction_id_1))
                .unwrap();
            store_engine
                .set(&kv_2.0, &kv_2.1, Some(transaction_id_2))
                .unwrap();
        }

        store_engine.commit_transaction(transaction_id_1).unwrap();
        store_engine.commit_transaction(transaction_id_2).unwrap();

        for (index, key) in shared_keys.iter().enumerate() {
            let actual_value = store_engine.get(key, None).unwrap().unwrap();
            let expected_value = key_value_pairs_2[index].1.clone();
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn read_inside_transaction() {
        let mut store_engine = get_store_engine("/tmp/test_read_inside_transaction");

        let key = KVKey::from("a");
        let value = KVValue::from("1");

        store_engine.set(&key, &value, None).unwrap();

        let transaction_id = store_engine.start_transaction().unwrap();

        {
            let actual_value = store_engine
                .get(&key, Some(transaction_id))
                .unwrap()
                .unwrap();
            let expected_value = value;

            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn dirty_read() {
        let mut store_engine = get_store_engine("/tmp/test_dirty_read");

        let key = KVKey::from("a");
        let origin_value = KVValue::from("1");
        let value_in_transaction = KVValue::from("2");

        store_engine.set(&key, &origin_value, None).unwrap();

        let transaction_id = store_engine.start_transaction().unwrap();

        {
            let actual_value = store_engine.get(&key, None).unwrap().unwrap();
            let expected_value = origin_value.clone();

            assert_eq!(actual_value, expected_value);
        }

        {
            store_engine
                .set(&key, &value_in_transaction, Some(transaction_id))
                .unwrap();
            let actual_value = store_engine.get(&key, None).unwrap().unwrap();
            let expected_value = origin_value.clone();

            assert_eq!(actual_value, expected_value);
        }

        store_engine.commit_transaction(transaction_id).unwrap();

        {
            let actual_value = store_engine.get(&key, None).unwrap().unwrap();
            let expected_value = value_in_transaction.clone();

            assert_eq!(actual_value, expected_value);
        }
    }

    // Inspired by Martin Kleppmann. “Designing Data-Intensive Applications.” Chapter 7
    #[test]
    fn test_repeatable_read() {
        let mut store_engine = get_store_engine("/tmp/test_repeatable_read");

        let account_1 = KVKey::from("account_1");
        let account_2 = KVKey::from("account_2");

        let original_value_1 = KVValue::new(&u64_to_u8_array(500));
        let original_value_2 = KVValue::new(&u64_to_u8_array(500));

        let target_value_1 = KVValue::new(&u64_to_u8_array(600));
        let target_value_2 = KVValue::new(&u64_to_u8_array(400));

        store_engine
            .set(&account_1, &original_value_1, None)
            .unwrap();
        store_engine
            .set(&account_2, &original_value_2, None)
            .unwrap();

        let transaction_id_1 = store_engine.start_transaction().unwrap();
        let transaction_id_2 = store_engine.start_transaction().unwrap();

        let transaction_1_account_1_value = store_engine
            .get(&account_1, Some(transaction_id_1))
            .unwrap()
            .unwrap();
        assert_eq!(transaction_1_account_1_value, original_value_1);

        store_engine
            .set(&account_1, &target_value_1, Some(transaction_id_2))
            .unwrap();

        store_engine
            .set(&account_2, &target_value_2, Some(transaction_id_2))
            .unwrap();

        store_engine.commit_transaction(transaction_id_2).unwrap();

        let transaction_1_account_2_value = store_engine
            .get(&account_2, Some(transaction_id_1))
            .unwrap()
            .unwrap();
        assert_eq!(transaction_1_account_2_value, original_value_2);
    }

    // Inspired by Martin Kleppmann. “Designing Data-Intensive Applications.” Chapter 7
    #[test]
    #[ignore]
    fn lost_update() {
        let mut store_engine = get_store_engine("/tmp/test_lost_update");

        let account = KVKey::from("account");
        let original_value = KVValue::new(&u64_to_u8_array(500));

        store_engine.set(&account, &original_value, None).unwrap();

        let transaction_id_1 = store_engine.start_transaction().unwrap();
        let transaction_id_2 = store_engine.start_transaction().unwrap();

        let transaction_1_account_value = store_engine
            .get(&account, Some(transaction_id_1))
            .unwrap()
            .unwrap();
        let transaction_2_account_value = store_engine
            .get(&account, Some(transaction_id_2))
            .unwrap()
            .unwrap();

        {
            let value_bytes = transaction_1_account_value.as_bytes();
            let mut value = u8_array_to_u64(&[
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
                value_bytes[4],
                value_bytes[5],
                value_bytes[6],
                value_bytes[7],
            ]);

            value += 100;

            let transaction_1_new_value = KVValue::new(&u64_to_u8_array(value));

            store_engine
                .set(&account, &transaction_1_new_value, Some(transaction_id_1))
                .unwrap();
        }

        {
            let value_bytes = transaction_2_account_value.as_bytes();
            let mut value = u8_array_to_u64(&[
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
                value_bytes[4],
                value_bytes[5],
                value_bytes[6],
                value_bytes[7],
            ]);

            value += 100;

            let transaction_2_new_value = KVValue::new(&u64_to_u8_array(value));

            store_engine
                .set(&account, &transaction_2_new_value, Some(transaction_id_2))
                .unwrap();
        }

        store_engine.commit_transaction(transaction_id_1).unwrap();
        store_engine.commit_transaction(transaction_id_2).unwrap();

        {
            let value_bytes = original_value.as_bytes();
            let mut value = u8_array_to_u64(&[
                value_bytes[0],
                value_bytes[1],
                value_bytes[2],
                value_bytes[3],
                value_bytes[4],
                value_bytes[5],
                value_bytes[6],
                value_bytes[7],
            ]);

            value += 200;

            let expected_value = KVValue::new(&u64_to_u8_array(value));
            let actual_value = store_engine.get(&account, None).unwrap().unwrap();

            assert_eq!(expected_value, actual_value);
        }
    }

    // Inspired by Martin Kleppmann. “Designing Data-Intensive Applications.” Chapter 7
    #[test]
    #[ignore]
    fn write_skew() {
        let mut store_engine = get_store_engine("/tmp/test_write_skew");

        let total_doctor_on_call = KVKey::from("total");
        let original_total_on_call_value = KVValue::new(&u64_to_u8_array(2));

        let is_alex_on_call = KVKey::from("Alex");
        let is_alex_on_call_value = KVValue::new(&[0x01]);
        let is_alex_on_call_target_value = KVValue::new(&[0x00]);

        let is_tom_on_call = KVKey::from("Tom");
        let is_tom_on_call_value = KVValue::new(&[0x01]);
        let is_tom_on_call_target_value = KVValue::new(&[0x00]);

        store_engine
            .set(&total_doctor_on_call, &original_total_on_call_value, None)
            .unwrap();
        store_engine
            .set(&is_alex_on_call, &is_alex_on_call_value, None)
            .unwrap();
        store_engine
            .set(&is_tom_on_call, &is_tom_on_call_value, None)
            .unwrap();

        let alex_transaction = store_engine.start_transaction().unwrap();
        let tom_transaction = store_engine.start_transaction().unwrap();

        {
            let total_on_call_value = store_engine
                .get(&total_doctor_on_call, Some(alex_transaction))
                .unwrap()
                .unwrap();
            let total_on_call_bytes = total_on_call_value.as_bytes();
            let mut total_on_call_num = u8_array_to_u64(&[
                total_on_call_bytes[0],
                total_on_call_bytes[1],
                total_on_call_bytes[2],
                total_on_call_bytes[3],
                total_on_call_bytes[4],
                total_on_call_bytes[5],
                total_on_call_bytes[6],
                total_on_call_bytes[7],
            ]);

            if total_on_call_num >= 2 {
                total_on_call_num -= 1;
                let new_total_on_call_value = KVValue::new(&u64_to_u8_array(total_on_call_num));
                store_engine
                    .set(
                        &total_doctor_on_call,
                        &new_total_on_call_value,
                        Some(alex_transaction),
                    )
                    .unwrap();
                store_engine
                    .set(
                        &is_alex_on_call,
                        &is_alex_on_call_target_value,
                        Some(alex_transaction),
                    )
                    .unwrap();
            }
        }

        {
            let total_on_call_value = store_engine
                .get(&total_doctor_on_call, Some(tom_transaction))
                .unwrap()
                .unwrap();
            let total_on_call_bytes = total_on_call_value.as_bytes();
            let mut total_on_call_num = u8_array_to_u64(&[
                total_on_call_bytes[0],
                total_on_call_bytes[1],
                total_on_call_bytes[2],
                total_on_call_bytes[3],
                total_on_call_bytes[4],
                total_on_call_bytes[5],
                total_on_call_bytes[6],
                total_on_call_bytes[7],
            ]);

            if total_on_call_num >= 2 {
                total_on_call_num -= 1;
                let new_total_on_call_value = KVValue::new(&u64_to_u8_array(total_on_call_num));
                store_engine
                    .set(
                        &total_doctor_on_call,
                        &new_total_on_call_value,
                        Some(tom_transaction),
                    )
                    .unwrap();
                store_engine
                    .set(
                        &is_tom_on_call,
                        &is_tom_on_call_target_value,
                        Some(tom_transaction),
                    )
                    .unwrap();
            }
        }

        store_engine.commit_transaction(tom_transaction).unwrap();

        assert!(store_engine.commit_transaction(alex_transaction).is_err());
    }
}
