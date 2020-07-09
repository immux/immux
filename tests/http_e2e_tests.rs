#[cfg(test)]
mod http_e2e_tests {
    use std::collections::HashMap;
    use std::thread;

    use immuxsys::constants as Constants;
    use immuxsys::storage::chain_height::ChainHeight;
    use immuxsys::storage::executor::unit_content::UnitContent;
    use immuxsys::storage::executor::unit_key::UnitKey;
    use immuxsys::storage::transaction_manager::TransactionId;
    use immuxsys_client::client::ImmuxDBClient;
    use immuxsys_dev_utils::dev_utils::{launch_db, notified_sleep};

    #[test]
    fn e2e_single_unit_get_set() {
        let port = 10083;
        thread::spawn(move || launch_db("e2e_single_unit_get_set", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();

        let collection = String::from("any_collection");
        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        for (key, content) in key_content_pairs.iter() {
            let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
            let expected_output = content.to_string();
            assert_eq!(actual_output, expected_output);
            assert_eq!(status_code.as_u16(), 200);
        }
    }

    #[test]
    fn e2e_revert_one() {
        let port = 10084;
        thread::spawn(move || launch_db("e2e_revert_one", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();

        let collection = String::from("any_collection");
        let unit_key = UnitKey::from("key1");
        let target_height = ChainHeight::new(3);

        let contents = vec![
            UnitContent::String(String::from("0")),
            UnitContent::String(String::from("1")),
            UnitContent::String(String::from("2")),
            UnitContent::String(String::from("3")),
            UnitContent::String(String::from("4")),
            UnitContent::String(String::from("5")),
        ];

        for content in contents.iter() {
            client.set_unit(&collection, &unit_key, &content).unwrap();
        }

        client.revert_one(&collection, &unit_key, &target_height).unwrap();
        let expected_output = &contents[target_height.as_u64() as usize].to_string();
        let (status_code, actual_output) = &client.get_by_key(&collection, &unit_key).unwrap();

        assert_eq!(actual_output, expected_output);
        assert_eq!(status_code.as_u16(), 200);
    }

    #[test]
    fn e2e_remove_one() {
        let port = 10085;
        thread::spawn(move || launch_db("e2e_remove_one", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 3;
        let (target_key, _target_content) = &key_content_pairs[target_pair_index];

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        client.remove_one(&collection, target_key).unwrap();

        for (key, content) in key_content_pairs.iter() {
            if key == target_key {
                let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
                assert_eq!(status_code.as_u16(), 200);
                assert!(actual_output.is_empty());
            } else {
                let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
                let expected_output = content.to_string();
                assert_eq!(status_code.as_u16(), 200);
                assert_eq!(actual_output, expected_output);
            }
        }
    }

    #[test]
    fn e2e_revert_all() {
        let port = 10086;
        thread::spawn(move || launch_db("e2e_revert_all", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        client.revert_all(&target_height).unwrap();

        for (index, (key, content)) in key_content_pairs.iter().enumerate() {
            if index <= target_height.as_u64() as usize {
                let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
                let expected_output = content.to_string();
                assert_eq!(status_code.as_u16(), 200);
                assert_eq!(actual_output, expected_output);
            } else {
                let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
                assert_eq!(status_code.as_u16(), 200);
                assert!(actual_output.is_empty());
            }
        }
    }

    #[test]
    fn e2e_remove_all() {
        let port = 10087;
        thread::spawn(move || launch_db("e2e_remove_all", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        client.remove_all().unwrap();

        for (key, _content) in key_content_pairs.iter() {
            let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
            assert_eq!(status_code.as_u16(), 200);
            assert!(actual_output.is_empty());
        }
    }

    #[test]
    fn e2e_atomicity_commit() {
        let port = 10088;
        thread::spawn(move || launch_db("e2e_atomicity_commit", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);

        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());
        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            client.transactional_set_unit(&collection, &key, &content, &transaction_id).unwrap();
        }

        client.commit_transaction(&transaction_id).unwrap();

        for (key, content) in key_content_pairs.iter() {
            let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
            let expected_output = content.to_string();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(actual_output, expected_output);
        }
    }

    #[test]
    fn e2e_atomicity_abort() {
        let port = 10089;
        thread::spawn(move || launch_db("e2e_atomicity_abort", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);

        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());
        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            client.transactional_set_unit(&collection, &key, &content, &transaction_id).unwrap();
        }

        client.abort_transaction(&transaction_id).unwrap();

        for (key, _content) in key_content_pairs.iter() {
            let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
            assert_eq!(status_code.as_u16(), 200);
            assert!(actual_output.is_empty());
        }
    }

    #[test]
    fn e2e_set_isolation() {
        let port = 10090;
        thread::spawn(move || launch_db("e2e_set_isolation", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let unit_key = UnitKey::from("key1");
        let content = UnitContent::String(String::from("This is string"));

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);

        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        client.transactional_set_unit(&collection, &unit_key, &content, &transaction_id).unwrap();

        {
            let (status_code, actual_output) = client.transactional_get(&collection, &unit_key, &transaction_id).unwrap();
            assert_eq!(status_code.as_u16(), 200);
            let expected_output = content.to_string();
            assert_eq!(actual_output, expected_output);
        }

        {
            let (status_code, output) = client.get_by_key(&collection, &unit_key).unwrap();
            assert_eq!(status_code.as_u16(), 200);
            assert!(output.is_empty());
        }

        client.commit_transaction(&transaction_id).unwrap();

        {
            let (status_code, actual_output) = client.get_by_key(&collection, &unit_key).unwrap();
            let expected_output = content.to_string();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(actual_output, expected_output);
        }
    }

    #[test]
    fn e2e_remove_one_isolation() {
        let port = 10091;
        thread::spawn(move || launch_db("e2e_remove_one_isolation", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let (target_key, _target_content) = &key_content_pairs[target_pair_index];

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);

        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        client.transactional_remove_one(&transaction_id, &collection, &target_key).unwrap();

        for (key, content) in key_content_pairs.iter() {
            let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
            let expected_output = content.to_string();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(actual_output, expected_output);
        }

        client.commit_transaction(&transaction_id).unwrap();

        for (key, content) in key_content_pairs.iter() {
            if key == target_key {
                let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
                assert_eq!(status_code.as_u16(), 200);
                assert!(actual_output.is_empty());
            } else {
                let (status_code, actual_output) = client.get_by_key(&collection, &key).unwrap();
                let expected_output = content.to_string();
                assert_eq!(status_code.as_u16(), 200);
                assert_eq!(actual_output, expected_output);
            }
        }
    }

    #[test]
    fn e2e_revert_one_isolation() {
        let port = 10092;
        thread::spawn(move || launch_db("e2e_revert_one_isolation", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key = UnitKey::from("key1");

        let target_pair_index = 2;
        let target_height = ChainHeight::new(target_pair_index);

        let contents = [
            UnitContent::String(String::from("this is a string")),
            UnitContent::Nil,
            UnitContent::Float64(12.0),
            UnitContent::Bool(true),
            UnitContent::Bool(false),
        ];

        for content in contents.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);

        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        client.transactional_revert_one(&collection, &key, &target_height, &transaction_id).unwrap();

        let (status_code, actual_output) = &client.get_by_key(&collection, &key).unwrap();
        let expected_output = &contents.last().unwrap().to_string();
        assert_eq!(status_code.as_u16(), 200);
        assert_eq!(actual_output, expected_output);

        client.commit_transaction(&transaction_id).unwrap();

        let (status_code, actual_output) = &client.get_by_key(&collection, &key).unwrap();
        let expected_output = &contents[target_pair_index as usize].to_string();
        assert_eq!(status_code.as_u16(), 200);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn e2e_remove_all_isolation() {
        let port = 10093;
        thread::spawn(move || launch_db("e2e_remove_all_isolation", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);

        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        client.remove_all().unwrap();

        for (key, _content) in key_content_pairs.iter() {
            let (status_code, actual_output) = client.transactional_get(&collection, &key, &transaction_id).unwrap();
            assert_eq!(status_code.as_u16(), 200);
            assert!(actual_output.is_empty())
        }
    }

    #[test]
    fn e2e_revert_all_isolation() {
        let port = 10094;
        thread::spawn(move || launch_db("e2e_revert_all_isolation", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 4;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);

        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        client.revert_all(&target_height).unwrap();

        for (index, (key, content)) in key_content_pairs.iter().enumerate() {
            if index <= target_height.as_u64() as usize {
                let (status_code, actual_output) = client.transactional_get(&collection, &key, &transaction_id).unwrap();
                let expected_output = content.to_string();
                assert_eq!(status_code.as_u16(), 200);
                assert_eq!(actual_output, expected_output);
            } else {
                let (status_code, actual_output) = client.transactional_get(&collection, &key, &transaction_id).unwrap();
                assert_eq!(status_code.as_u16(), 200);
                assert!(actual_output.is_empty());
            }
        }
    }

    #[test]
    fn e2e_transaction_not_alive_after_revert_all() {
        let port = 10095;
        thread::spawn(move || launch_db("e2e_transaction_not_alive_after_revert_all", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            client.set_unit(&collection, &key, &content).unwrap();
        }

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);
        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        client.revert_all(&target_height).unwrap();

        let (status_code, _) = client.commit_transaction(&transaction_id).unwrap();

        assert_eq!(status_code.as_u16(), 500);
    }

    #[test]
    fn e2e_unexpected_commit_transaction_id() {
        let port = 10096;
        thread::spawn(move || launch_db("e2e_unexpected_commit_transaction_id", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();

        let fake_transaction_id = TransactionId::new(100);

        let (status_code, _) = client.commit_transaction(&fake_transaction_id).unwrap();
        assert_eq!(status_code.as_u16(), 500);
    }

    #[test]
    fn e2e_unexpected_abort_transaction_id() {
        let port = 10097;
        thread::spawn(move || launch_db("e2e_unexpected_abort_transaction_id", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();

        let fake_transaction_id = TransactionId::new(100);

        let (status_code, _) = client.abort_transaction(&fake_transaction_id).unwrap();
        assert_eq!(status_code.as_u16(), 500);
    }

    #[test]
    fn e2e_last_one_commit_wins() {
        let port = 10098;
        thread::spawn(move || launch_db("e2e_transaction_not_alive_after_revert_all", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let shared_keys = [UnitKey::from("a"), UnitKey::from("b"), UnitKey::from("c")];

        let key_value_pairs_1: Vec<(UnitKey, UnitContent)> = shared_keys
            .iter()
            .enumerate()
            .map(|(index, key)| {
                let value = format!("{:?}", index + 1);
                (key.clone(), UnitContent::String(value))
            })
            .collect();

        let key_value_pairs_2: Vec<(UnitKey, UnitContent)> = shared_keys
            .iter()
            .enumerate()
            .map(|(index, key)| {
                let value = format!("{:?}", (index + 1) * 100);
                (key.clone(), UnitContent::String(value))
            })
            .collect();

        let mixed_kv_pairs: Vec<_> = key_value_pairs_1.iter().zip(&key_value_pairs_2).collect();

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);
        let transaction_id_1 = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);
        let transaction_id_2 = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        for kv_pairs in mixed_kv_pairs {
            let kv_1 = kv_pairs.0;
            let kv_2 = kv_pairs.1;
            client.transactional_set_unit(&collection, &kv_1.0, &kv_1.1, &transaction_id_1).unwrap();
            client.transactional_set_unit(&collection, &kv_2.0, &kv_2.1, &transaction_id_2).unwrap();
        }

        client.commit_transaction(&transaction_id_1).unwrap();
        client.commit_transaction(&transaction_id_2).unwrap();

        for (index, key) in shared_keys.iter().enumerate() {
            let (status_code, actual_value) = client.get_by_key(&collection, key).unwrap();
            let expected_value = key_value_pairs_2[index].1.clone();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(actual_value, expected_value.to_string());
        }
    }

    #[test]
    fn e2e_read_inside_transaction() {
        let port = 10099;
        thread::spawn(move || launch_db("e2e_read_inside_transaction", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key = UnitKey::from("a");
        let value = UnitContent::String(String::from("1"));

        client.set_unit(&collection, &key, &value).unwrap();

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);
        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        {
            let (status_code, actual_value) = client.transactional_get(&collection, &key, &transaction_id).unwrap();
            let expected_value = value.to_string();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(actual_value, expected_value);
        }
    }

    #[test]
    fn e2e_dirty_read() {
        let port = 10100;
        thread::spawn(move || launch_db("e2e_read_inside_transaction", port));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBClient::new(host).unwrap();
        let collection = String::from("any_collection");

        let key = UnitKey::from("a");
        let origin_value = UnitContent::String(String::from("1"));
        let value_in_transaction =  UnitContent::String(String::from("2"));

        client.set_unit(&collection, &key, &origin_value).unwrap();

        let (status_code, transaction_id_str) = client.create_transaction().unwrap();
        assert_eq!(status_code.as_u16(), 200);
        let transaction_id = TransactionId::new(transaction_id_str.parse::<u64>().unwrap());

        {
            let (status_code, actual_value) = client.get_by_key(&collection, &key).unwrap();
            let expected_value = &origin_value.to_string();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(&actual_value, expected_value);
        }

        {
            client.transactional_set_unit(&collection, &key, &value_in_transaction, &transaction_id).unwrap();

            let (status_code, actual_value) = client.get_by_key(&collection,&key).unwrap();
            let expected_value = &origin_value.to_string();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(&actual_value, expected_value);
        }

        client.commit_transaction(&transaction_id).unwrap();

        {
            let (status_code, actual_value) = client.transactional_get(&collection, &key, &transaction_id).unwrap();
            let expected_value = &value_in_transaction.to_string();
            assert_eq!(status_code.as_u16(), 200);
            assert_eq!(&actual_value, expected_value);
        }
    }

    fn get_key_content_pairs() -> Vec<(UnitKey, UnitContent)> {
        let mut map = HashMap::new();
        map.insert(String::from("key1"), UnitContent::String(String::from("string in map")));
        map.insert(String::from("key2"), UnitContent::Nil);
        map.insert(String::from("key3"), UnitContent::Bool(false));

        [
            (UnitKey::from("key1"), UnitContent::String(String::from("this is a string"))),
            (UnitKey::from("key2"), UnitContent::Nil),
            (UnitKey::from("key3"), UnitContent::Float64(12.0)),
            (UnitKey::from("key4"), UnitContent::Bool(true)),
            (UnitKey::from("key5"), UnitContent::Bool(false)),
            (
                UnitKey::from("key6"),
                UnitContent::Array(vec![
                    UnitContent::String(String::from("string in an array")),
                    UnitContent::Nil,
                    UnitContent::Bool(true),
                    UnitContent::Float64(12.0),
                ]),
            ),
            (UnitKey::from("key7"), UnitContent::Map(map)),
        ]
        .to_vec()
    }
}
