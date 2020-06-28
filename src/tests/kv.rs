#[cfg(test)]
mod kv_tests {

    use std::collections::HashMap;
    use std::fs::remove_file;
    use std::path::PathBuf;

    use immuxsys::storage::chain_height::ChainHeight;
    use immuxsys::storage::kv::{get_log_file_dir, LogKeyValueStore};
    use immuxsys::storage::kvkey::KVKey;
    use immuxsys::storage::kvvalue::KVValue;
    use immuxsys::storage::transaction_manager::TransactionId;
    use immuxsys::utils::{u64_to_u8_array, u8_array_to_u64};

    fn get_store_engine(path: &PathBuf) -> LogKeyValueStore {
        let log_file_path = get_log_file_dir(&path);

        if log_file_path.exists() {
            remove_file(log_file_path).unwrap();
        }

        let store_engine = LogKeyValueStore::open(&path).unwrap();

        return store_engine;
    }

    #[test]
    fn kv_set() {
        let path = PathBuf::from("/tmp/test_set");
        let mut store_engine = get_store_engine(&path);

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let expected_value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);

        store_engine
            .set(key.clone(), expected_value.clone(), None)
            .unwrap();
        let actual_value = store_engine.get(&key, &None).unwrap().unwrap();

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn kv_revert_one() {
        let path = PathBuf::from("/tmp/test_revert_one");
        let mut store_engine = get_store_engine(&path);
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
            store_engine.set(key.clone(), value.clone(), None).unwrap();
        }

        store_engine
            .revert_one(key.clone(), &target_height, None)
            .unwrap();

        let actual_output = store_engine.get(&key, &None).unwrap().unwrap();

        let expected_output = &values.as_slice()[target_height.as_u64() as usize];

        assert_eq!(&actual_output, expected_output);
    }

    #[test]
    fn kv_revert_all() {
        let path = PathBuf::from("/tmp/test_revert_all");
        let mut store_engine = get_store_engine(&path);
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
            store_engine
                .set(kv_pair.0.clone(), kv_pair.1.clone(), None)
                .unwrap();
        }

        store_engine.revert_all(&target_height).unwrap();

        let mut expected_hashmap = HashMap::new();

        for expected_kv_pair in &key_value_pairs[..target_height.as_u64() as usize] {
            expected_hashmap.insert(&expected_kv_pair.0, &expected_kv_pair.1);
        }

        for (key, expected_value) in expected_hashmap {
            let actual_value = store_engine.get(&key, &None).unwrap().unwrap();
            assert_eq!(expected_value, &actual_value);
        }
    }

    #[test]
    fn kv_remove() {
        let path = PathBuf::from("/tmp/test_remove");
        let mut store_engine = get_store_engine(&path);

        let key = KVKey::new(&[0x00, 0x01, 0x03]);
        let expected_value = KVValue::new(&[0xff, 0xff, 0xff, 0xff]);

        store_engine
            .set(key.clone(), expected_value.clone(), None)
            .unwrap();
        store_engine.remove_one(key.clone(), None).unwrap();

        let actual_value = store_engine.get(&key, &None).unwrap();

        assert_eq!(actual_value, None);
    }

    #[test]
    fn kv_remove_all() {
        let path = PathBuf::from("/tmp/test_remove_all");
        let mut store_engine = get_store_engine(&path);

        let key_value_pairs = vec![
            (KVKey::new(&[0xff]), KVValue::new(&[0x00])),
            (KVKey::new(&[0xf2]), KVValue::new(&[0xff])),
            (KVKey::new(&[0x23]), KVValue::new(&[0x22])),
            (KVKey::new(&[0x11]), KVValue::new(&[0x01])),
        ];

        for kv_pair in &key_value_pairs {
            store_engine
                .set(kv_pair.0.clone(), kv_pair.1.clone(), None)
                .unwrap();
        }

        store_engine.remove_all().unwrap();

        for kv_pair in &key_value_pairs {
            let out_put = store_engine.get(&kv_pair.0, &None).unwrap();
            assert_eq!(out_put, None);
        }
    }

    #[test]
    fn kv_atomicity_commit() {
        let path = PathBuf::from("/tmp/test_atomicity_commit");
        let mut store_engine = get_store_engine(&path);

        let key_value_paris = [
            (KVKey::from("a"), KVValue::from("1")),
            (KVKey::from("b"), KVValue::from("2")),
            (KVKey::from("c"), KVValue::from("3")),
            (KVKey::from("d"), KVValue::from("4")),
        ];

        let transaction_id = store_engine.start_transaction().unwrap();
        for pair in &key_value_paris {
            store_engine
                .set(pair.0.clone(), pair.1.clone(), Some(transaction_id))
                .unwrap();
        }
        store_engine.commit_transaction(transaction_id).unwrap();

        for pair in &key_value_paris {
            let actual_value = store_engine.get(&pair.0, &None).unwrap().unwrap();
            let expected_value = pair.1.clone();

            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn kv_atomicity_abort() {
        let path = PathBuf::from("/tmp/test_atomicity_abort");
        let mut store_engine = get_store_engine(&path);

        let key_value_paris = [
            (KVKey::from("1"), KVValue::from("a")),
            (KVKey::from("2"), KVValue::from("b")),
            (KVKey::from("3"), KVValue::from("c")),
        ];

        let expected_value = None;

        let transaction_id = store_engine.start_transaction().unwrap();
        for pair in &key_value_paris {
            store_engine
                .set(pair.0.clone(), pair.1.clone(), Some(transaction_id))
                .unwrap();
        }
        store_engine.abort_transaction(transaction_id).unwrap();

        for pair in &key_value_paris {
            let actual_value = store_engine.get(&pair.0, &None).unwrap();
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn kv_set_isolation() {
        let path = PathBuf::from("/tmp/test_set_isolation");
        let mut store_engine = get_store_engine(&path);

        let key = KVKey::from("a");
        let value = KVValue::from("1");

        let transaction_id = store_engine.start_transaction().unwrap();
        store_engine
            .set(key.clone(), value.clone(), Some(transaction_id))
            .unwrap();

        {
            let actual_value_within_transaction = store_engine
                .get(&key, &Some(transaction_id))
                .unwrap()
                .unwrap();
            let expected_value_within_transaction = value.clone();

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        {
            let actual_value_outside_transaction = store_engine.get(&key, &None).unwrap();
            let expected_value_outside_transaction = None;

            assert_eq!(
                expected_value_outside_transaction,
                actual_value_outside_transaction
            );
        }

        store_engine.commit_transaction(transaction_id).unwrap();

        {
            let actual_value_after_commit = store_engine.get(&key, &None).unwrap().unwrap();
            let expected_value_after_commit = value.clone();

            assert_eq!(expected_value_after_commit, actual_value_after_commit);
        }
    }

    #[test]
    fn kv_remove_one_isolation() {
        let path = PathBuf::from("/tmp/test_remove_one_isolation");
        let mut store_engine = get_store_engine(&path);

        let key_value_paris = [
            (KVKey::from("one"), KVValue::from("a")),
            (KVKey::from("two"), KVValue::from("b")),
            (KVKey::from("three"), KVValue::from("c")),
        ];

        let kv_to_be_removed = &key_value_paris[0];

        for pair in &key_value_paris {
            store_engine
                .set(pair.0.clone(), pair.1.clone(), None)
                .unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();
        store_engine
            .remove_one(kv_to_be_removed.0.clone(), Some(transaction_id))
            .unwrap();

        {
            let actual_value_within_transaction = store_engine
                .get(&kv_to_be_removed.0.clone(), &Some(transaction_id))
                .unwrap();
            let expected_value_within_transaction = None;

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        {
            let actual_value_outside_transaction = store_engine
                .get(&&kv_to_be_removed.0.clone(), &None)
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
            let actual_value_after_commit = store_engine
                .get(&kv_to_be_removed.0.clone(), &None)
                .unwrap();
            let expected_value_after_commit = None;

            assert_eq!(expected_value_after_commit, actual_value_after_commit);
        }
    }

    #[test]
    fn kv_revert_one_isolation() {
        let path = PathBuf::from("/tmp/test_revert_one_isolation");
        let mut store_engine = get_store_engine(&path);

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
            store_engine
                .set(pair.0.clone(), pair.1.clone(), None)
                .unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();
        store_engine
            .revert_one(
                kv_to_be_reverted.0.clone(),
                &target_height,
                Some(transaction_id),
            )
            .unwrap();

        {
            let actual_value_within_transaction = store_engine
                .get(&kv_to_be_reverted.0.clone(), &Some(transaction_id))
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
                .get(&kv_to_be_reverted.0.clone(), &None)
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
                .get(&kv_to_be_reverted.0.clone(), &None)
                .unwrap()
                .unwrap();
            let expected_value_after_commit = kv_to_be_reverted.1.clone();

            assert_eq!(expected_value_after_commit, actual_value_after_commit);
        }
    }

    #[test]
    fn kv_remove_all_isolation() {
        let path = PathBuf::from("/tmp/test_remove_all_isolation");
        let mut store_engine = get_store_engine(&path);

        let key_value_paris = [
            (KVKey::from("k1"), KVValue::from("a")),
            (KVKey::from("k2"), KVValue::from("b")),
            (KVKey::from("k3"), KVValue::from("c")),
        ];

        let target_kv = (KVKey::from("k4"), KVValue::from("d"));

        for pair in &key_value_paris {
            store_engine
                .set(pair.0.clone(), pair.1.clone(), None)
                .unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();

        {
            store_engine
                .set(
                    target_kv.0.clone(),
                    target_kv.1.clone(),
                    Some(transaction_id),
                )
                .unwrap();
            let actual_value_within_transaction = store_engine
                .get(&target_kv.0.clone(), &Some(transaction_id))
                .unwrap()
                .unwrap();
            let expected_value_within_transaction = target_kv.1.clone();

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        store_engine.remove_all().unwrap();

        {
            for pair in &key_value_paris {
                let actual_value_outside_transaction =
                    store_engine.get(&pair.0.clone(), &None).unwrap();
                let expected_value_outside_transaction = None;

                assert_eq!(
                    actual_value_outside_transaction,
                    expected_value_outside_transaction
                );
            }
        }

        store_engine.commit_transaction(transaction_id).unwrap();

        {
            let actual_value_after_commit = store_engine
                .get(&target_kv.0.clone(), &None)
                .unwrap()
                .unwrap();
            let expected_value_after_commit = target_kv.1.clone();

            assert_eq!(expected_value_after_commit, actual_value_after_commit);
        }
    }

    #[test]
    fn kv_revert_all_isolation() {
        let path = PathBuf::from("/tmp/test_revert_all_isolation");
        let mut store_engine = get_store_engine(&path);

        let key_value_pairs = [
            (KVKey::from("k1"), KVValue::from("a")),
            (KVKey::from("k1"), KVValue::from("b")),
            (KVKey::from("k1"), KVValue::from("c")),
            (KVKey::from("k1"), KVValue::from("d")),
            (KVKey::from("k1"), KVValue::from("e")),
            (KVKey::from("k2"), KVValue::from("f")),
            (KVKey::from("k3"), KVValue::from("g")),
            (KVKey::from("k4"), KVValue::from("h")),
            (KVKey::from("k1"), KVValue::from("i")),
            (KVKey::from("k1"), KVValue::from("j")),
        ];

        let index = 6;
        let target_height = ChainHeight::new(index);

        for pair in &key_value_pairs {
            store_engine
                .set(pair.0.clone(), pair.1.clone(), None)
                .unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();

        let target_kv = (KVKey::from("k100"), KVValue::from("z"));

        {
            store_engine
                .set(
                    target_kv.0.clone(),
                    target_kv.1.clone(),
                    Some(transaction_id),
                )
                .unwrap();
            let expected_value_within_transaction = target_kv.1.clone();
            let actual_value_within_transaction = store_engine
                .get(&target_kv.0.clone(), &Some(transaction_id))
                .unwrap()
                .unwrap();

            assert_eq!(
                expected_value_within_transaction,
                actual_value_within_transaction
            );
        }

        store_engine.revert_all(&target_height).unwrap();

        {
            let mut expected_hashmap = HashMap::new();

            for expected_kv_pair in &key_value_pairs[..target_height.as_u64() as usize] {
                expected_hashmap.insert(&expected_kv_pair.0, &expected_kv_pair.1);
            }

            for (key, expected_value_after_revert) in expected_hashmap {
                let actual_value_after_revert = store_engine.get(&key, &None).unwrap().unwrap();
                assert_eq!(expected_value_after_revert, &actual_value_after_revert);
            }
        }

        {
            let actual_target_value_after_revert =
                store_engine.get(&target_kv.0.clone(), &None).unwrap();
            let expected_target_value_after_revert = None;

            assert_eq!(
                expected_target_value_after_revert,
                actual_target_value_after_revert
            );
        }
    }

    #[test]
    fn transaction_not_alive_after_revert_all() {
        let path = PathBuf::from("/tmp/test_revert_all_transaction_not_alive");
        let mut store_engine = get_store_engine(&path);

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
            store_engine
                .set(pair.0.clone(), pair.1.clone(), None)
                .unwrap();
        }

        let transaction_id = store_engine.start_transaction().unwrap();

        store_engine.revert_all(&target_height).unwrap();

        assert!(store_engine.commit_transaction(transaction_id).is_err());
    }

    #[test]
    #[should_panic]
    fn unexpected_commit_transaction_id() {
        let path = PathBuf::from("/tmp/test_unexpected_commit_transaction_id");
        let mut store_engine = get_store_engine(&path);

        let some_random_transaction_id = 100;

        store_engine
            .commit_transaction(TransactionId::new(some_random_transaction_id))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn unexpected_abort_transaction_id() {
        let path = PathBuf::from("/tmp/test_unexpected_abort_transaction_id");
        let mut store_engine = get_store_engine(&path);

        let some_random_transaction_id = 10;

        store_engine
            .abort_transaction(TransactionId::new(some_random_transaction_id))
            .unwrap();
    }

    #[test]
    fn last_one_commit_wins() {
        let path = PathBuf::from("/tmp/test_last_one_commit_wins");
        let mut store_engine = get_store_engine(&path);

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
                .set(kv_1.0.clone(), kv_1.1.clone(), Some(transaction_id_1))
                .unwrap();
            store_engine
                .set(kv_2.0.clone(), kv_2.1.clone(), Some(transaction_id_2))
                .unwrap();
        }

        store_engine.commit_transaction(transaction_id_1).unwrap();
        store_engine.commit_transaction(transaction_id_2).unwrap();

        for (index, key) in shared_keys.iter().enumerate() {
            let actual_value = store_engine.get(key, &None).unwrap().unwrap();
            let expected_value = key_value_pairs_2[index].1.clone();
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn read_inside_transaction() {
        let path = PathBuf::from("/tmp/test_read_inside_transaction");
        let mut store_engine = get_store_engine(&path);

        let key = KVKey::from("a");
        let value = KVValue::from("1");

        store_engine.set(key.clone(), value.clone(), None).unwrap();

        let transaction_id = store_engine.start_transaction().unwrap();

        {
            let actual_value = store_engine
                .get(&key, &Some(transaction_id))
                .unwrap()
                .unwrap();
            let expected_value = value;

            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn dirty_read() {
        let path = PathBuf::from("/tmp/test_dirty_read");
        let mut store_engine = get_store_engine(&path);

        let key = KVKey::from("a");
        let origin_value = KVValue::from("1");
        let value_in_transaction = KVValue::from("2");

        store_engine
            .set(key.clone(), origin_value.clone(), None)
            .unwrap();

        let transaction_id = store_engine.start_transaction().unwrap();

        {
            let actual_value = store_engine.get(&key, &None).unwrap().unwrap();
            let expected_value = origin_value.clone();

            assert_eq!(actual_value, expected_value);
        }

        {
            store_engine
                .set(
                    key.clone(),
                    value_in_transaction.clone(),
                    Some(transaction_id),
                )
                .unwrap();
            let actual_value = store_engine.get(&key, &None).unwrap().unwrap();
            let expected_value = origin_value.clone();

            assert_eq!(actual_value, expected_value);
        }

        store_engine.commit_transaction(transaction_id).unwrap();

        {
            let actual_value = store_engine.get(&key, &None).unwrap().unwrap();
            let expected_value = value_in_transaction.clone();

            assert_eq!(actual_value, expected_value);
        }
    }

    // Inspired by Martin Kleppmann. “Designing Data-Intensive Applications.” Chapter 7
    #[test]
    #[ignore]
    fn repeatable_read() {
        let path = PathBuf::from("/tmp/test_repeatable_read");
        let mut store_engine = get_store_engine(&path);

        let account_1 = KVKey::from("account_1");
        let account_2 = KVKey::from("account_2");

        let original_value_1 = KVValue::new(&u64_to_u8_array(500));
        let original_value_2 = KVValue::new(&u64_to_u8_array(500));

        let target_value_1 = KVValue::new(&u64_to_u8_array(600));
        let target_value_2 = KVValue::new(&u64_to_u8_array(400));

        store_engine
            .set(account_1.clone(), original_value_1.clone(), None)
            .unwrap();
        store_engine
            .set(account_2.clone(), original_value_2.clone(), None)
            .unwrap();

        let transaction_id_1 = store_engine.start_transaction().unwrap();
        let transaction_id_2 = store_engine.start_transaction().unwrap();

        let transaction_1_account_1_value = store_engine
            .get(&account_1, &Some(transaction_id_1))
            .unwrap()
            .unwrap();
        assert_eq!(transaction_1_account_1_value, original_value_1);

        store_engine
            .set(
                account_1.clone(),
                target_value_1.clone(),
                Some(transaction_id_2),
            )
            .unwrap();
        store_engine
            .set(
                account_2.clone(),
                target_value_2.clone(),
                Some(transaction_id_2),
            )
            .unwrap();

        store_engine.commit_transaction(transaction_id_2).unwrap();

        let transaction_1_account_2_value = store_engine
            .get(&account_2, &Some(transaction_id_1))
            .unwrap()
            .unwrap();
        assert_eq!(transaction_1_account_2_value, original_value_2);
    }

    // Inspired by Martin Kleppmann. “Designing Data-Intensive Applications.” Chapter 7
    #[test]
    #[ignore]
    fn lost_update() {
        let path = PathBuf::from("/tmp/test_lost_update");
        let mut store_engine = get_store_engine(&path);

        let account = KVKey::from("account");
        let original_value = KVValue::new(&u64_to_u8_array(500));

        store_engine
            .set(account.clone(), original_value.clone(), None)
            .unwrap();

        let transaction_id_1 = store_engine.start_transaction().unwrap();
        let transaction_id_2 = store_engine.start_transaction().unwrap();

        let transaction_1_account_value = store_engine
            .get(&account, &Some(transaction_id_1))
            .unwrap()
            .unwrap();
        let transaction_2_account_value = store_engine
            .get(&account, &Some(transaction_id_2))
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
                .set(
                    account.clone(),
                    transaction_1_new_value,
                    Some(transaction_id_1),
                )
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
                .set(
                    account.clone(),
                    transaction_2_new_value,
                    Some(transaction_id_2),
                )
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
            let actual_value = store_engine.get(&account, &None).unwrap().unwrap();

            assert_eq!(expected_value, actual_value);
        }
    }

    // Inspired by Martin Kleppmann. “Designing Data-Intensive Applications.” Chapter 7
    #[test]
    #[ignore]
    fn write_skew() {
        let path = PathBuf::from("/tmp/test_write_skew");
        let mut store_engine = get_store_engine(&path);

        let total_doctor_on_call = KVKey::from("total");
        let original_total_on_call_value = KVValue::new(&u64_to_u8_array(2));

        let is_alex_on_call = KVKey::from("Alex");
        let is_alex_on_call_value = KVValue::new(&[0x01]);
        let is_alex_on_call_target_value = KVValue::new(&[0x00]);

        let is_tom_on_call = KVKey::from("Tom");
        let is_tom_on_call_value = KVValue::new(&[0x01]);
        let is_tom_on_call_target_value = KVValue::new(&[0x00]);

        store_engine
            .set(
                total_doctor_on_call.clone(),
                original_total_on_call_value.clone(),
                None,
            )
            .unwrap();
        store_engine
            .set(is_alex_on_call.clone(), is_alex_on_call_value.clone(), None)
            .unwrap();
        store_engine
            .set(is_tom_on_call.clone(), is_tom_on_call_value.clone(), None)
            .unwrap();

        let alex_transaction = store_engine.start_transaction().unwrap();
        let tom_transaction = store_engine.start_transaction().unwrap();

        {
            let total_on_call_value = store_engine
                .get(&total_doctor_on_call, &Some(alex_transaction))
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
                        total_doctor_on_call.clone(),
                        new_total_on_call_value.clone(),
                        Some(alex_transaction),
                    )
                    .unwrap();
                store_engine
                    .set(
                        is_alex_on_call,
                        is_alex_on_call_target_value.clone(),
                        Some(alex_transaction),
                    )
                    .unwrap();
            }
        }

        {
            let total_on_call_value = store_engine
                .get(&total_doctor_on_call, &Some(tom_transaction))
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
                        total_doctor_on_call.clone(),
                        new_total_on_call_value.clone(),
                        Some(tom_transaction),
                    )
                    .unwrap();
                store_engine
                    .set(
                        is_tom_on_call,
                        is_tom_on_call_target_value.clone(),
                        Some(tom_transaction),
                    )
                    .unwrap();
            }
        }

        store_engine.commit_transaction(tom_transaction).unwrap();

        assert!(store_engine.commit_transaction(alex_transaction).is_err());
    }
}
