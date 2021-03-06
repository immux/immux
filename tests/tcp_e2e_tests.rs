#[cfg(test)]
mod tcp_e2e_tests {
    use std::error::Error;

    use immuxsys::constants as Constants;
    use immuxsys::server::errors::ServerError;
    use immuxsys::server::tcp_response::TcpResponse;
    use immuxsys::storage::chain_height::ChainHeight;
    use immuxsys::storage::errors::KVError::TransactionManagerError;
    use immuxsys::storage::executor::errors::ExecutorError::KVError;
    use immuxsys::storage::executor::grouping_label::GroupingLabel;
    use immuxsys::storage::executor::outcome::Outcome;
    use immuxsys::storage::executor::unit_content::UnitContent;
    use immuxsys::storage::executor::unit_key::UnitKey;
    use immuxsys::storage::transaction_manager::TransactionId;
    use immuxsys::storage::transaction_manager::TransactionManagerError::TransactionNotAlive;
    use immuxsys_client::tcp_client::ImmuxDBTcpClient;
    use immuxsys_client::ImmuxDBClient;
    use immuxsys_dev_utils::data_models::berka99::{
        Account, Card, Client, Disp, District, Loan, Order, Trans,
    };
    use immuxsys_dev_utils::data_models::business::Business;
    use immuxsys_dev_utils::data_models::census90::CensusEntry;
    use immuxsys_dev_utils::data_models::covid::Covid;
    use immuxsys_dev_utils::dev_utils::{
        csv_to_json_table, get_key_content_pairs, get_phone_mode_test_predicates,
        get_phone_model_fixture, launch_test_db_servers, UnitList,
    };

    #[test]
    fn tcp_e2e_remove_groupings() {
        let port = 7998;
        launch_test_db_servers("tcp_e2e_remove_groupings", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let expected_groupings = vec![
            GroupingLabel::from("grouping1"),
            GroupingLabel::from("grouping2"),
            GroupingLabel::from("grouping3"),
            GroupingLabel::from("grouping4"),
            GroupingLabel::from("grouping5"),
            GroupingLabel::from("grouping6"),
        ];

        let random_key = UnitKey::from("some random key");
        let random_content = UnitContent::String(String::from("some random value"));

        for grouping in expected_groupings.iter() {
            let response = client
                .set_unit(&grouping, &random_key, &random_content)
                .unwrap();
            let actual_outcome = Outcome::InsertSuccess;
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, actual_outcome),
                TcpResponse::ServerError(_) => panic!(),
            }
        }

        let target_groupings = vec![
            expected_groupings[0].clone(),
            expected_groupings[1].clone(),
            expected_groupings[2].clone(),
        ];
        let response = client.remove_groupings(&target_groupings).unwrap();
        let actual_outcome = Outcome::DeleteGroupingSuccess;

        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, actual_outcome);
            }
            TcpResponse::ServerError(_) => panic!(),
        }

        for grouping in target_groupings.iter() {
            let response = client.get_by_key(grouping, &random_key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    let actual_outcome = Outcome::Select(vec![]);
                    assert_eq!(outcome, actual_outcome);
                }
                TcpResponse::ServerError(_) => panic!(),
            }
        }

        let response = client.get_all_groupings().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => match outcome {
                Outcome::GetAllGroupingsSuccess(actual_groupings) => {
                    for grouping in actual_groupings.iter() {
                        assert!(expected_groupings.contains(grouping));
                    }

                    for grouping in target_groupings.iter() {
                        assert!(!actual_groupings.contains(grouping));
                    }
                }
                _ => panic!("tcp_e2e_remove_groupings failed"),
            },
            _ => panic!("tcp_e2e_remove_groupings failed"),
        }
    }

    #[test]
    fn tcp_e2e_get_groupings() {
        let port = 7999;
        launch_test_db_servers("tcp_e2e_get_groupings", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let expected_groupings = vec![
            GroupingLabel::from("grouping1"),
            GroupingLabel::from("grouping2"),
            GroupingLabel::from("grouping3"),
            GroupingLabel::from("grouping4"),
            GroupingLabel::from("grouping5"),
            GroupingLabel::from("grouping6"),
        ];

        let random_key = UnitKey::from("random key");
        let random_content = UnitContent::String(String::from("hello world"));

        for grouping in expected_groupings.iter() {
            let response = client
                .set_unit(&grouping, &random_key, &random_content)
                .unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::InsertSuccess);
                }
                TcpResponse::ServerError(_) => panic!(),
            }
        }

        let response = client.get_all_groupings().unwrap();

        match response {
            TcpResponse::Outcome(outcome) => match outcome {
                Outcome::GetAllGroupingsSuccess(actual_groupings) => {
                    for grouping in actual_groupings.iter() {
                        assert!(expected_groupings.contains(grouping));
                    }

                    for grouping in expected_groupings.iter() {
                        assert!(actual_groupings.contains(grouping));
                    }

                    assert_eq!(actual_groupings.len(), expected_groupings.len());
                }
                _ => panic!("tcp_e2e_get_groupings failed"),
            },
            _ => panic!("tcp_e2e_get_groupings failed"),
        }
    }

    #[test]
    fn tcp_e2e_grouping_get_set() {
        let port = 8000;
        launch_test_db_servers("tcp_e2e_grouping_get_set", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let unit_key = UnitKey::new("key".as_bytes());
        let content = UnitContent::String("content".to_string());

        let response = client.set_unit(&grouping, &unit_key, &content).unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
            _ => panic!(),
        }

        let grouping = GroupingLabel::new("the_other_grouping".as_bytes());
        let response = client.get_by_key(&grouping, &unit_key).unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::Select(vec![])),
            _ => panic!(),
        }
    }

    #[test]
    fn tcp_e2e_single_unit_get_set() {
        let port = 8001;
        launch_test_db_servers("tcp_e2e_single_unit_get_set", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = GroupingLabel::new("a".as_bytes());
        let unit_key = UnitKey::new("key".as_bytes());
        let unit_content = UnitContent::String("content".to_string());

        let response = client
            .set_unit(&grouping, &unit_key, &unit_content)
            .unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
            _ => panic!(),
        }

        let response = client.get_by_key(&grouping, &unit_key).unwrap();
        let expected_output = Outcome::Select(vec![unit_content]);

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(expected_output, outcome),
            _ => panic!(),
        }
    }

    #[test]
    fn tcp_e2e_revert_one() {
        let port = 8002;
        launch_test_db_servers("tcp_e2e_revert_one", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = GroupingLabel::new("any_grouping".as_bytes());
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
            let response = client.set_unit(&grouping, &unit_key, content).unwrap();

            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let response = client
            .revert_one(&grouping, &unit_key, &target_height)
            .unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::RevertOneSuccess),
            _ => panic!(),
        }

        let response = client.get_by_key(&grouping, &unit_key).unwrap();

        let expected_content = &contents[target_height.as_u64() as usize];

        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::Select(vec![expected_content.clone()]));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn tcp_e2e_real_data_get_set() {
        let port = 8003;
        launch_test_db_servers("tcp_e2e_real_data_get_set", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let paths = [
            "dev_utils/src/data_models/data-raw/account.asc",
            "dev_utils/src/data_models/data-raw/card.asc",
            "dev_utils/src/data_models/data-raw/client.asc",
            "dev_utils/src/data_models/data-raw/disp.asc",
            "dev_utils/src/data_models/data-raw/district.asc",
            "dev_utils/src/data_models/data-raw/loan.asc",
            "dev_utils/src/data_models/data-raw/order.asc",
            "dev_utils/src/data_models/data-raw/trans.asc",
            "dev_utils/src/data_models/data-raw/anzsic06.csv",
            "dev_utils/src/data_models/data-raw/census90.txt",
            "dev_utils/src/data_models/data-raw/covid.csv",
        ];
        let row_limit = 1000;

        let dataset: Vec<(String, UnitList)> = paths
            .iter()
            .map(|path| -> (String, Result<UnitList, Box<dyn Error>>) {
                let path_segments: Vec<&str> = path.split("/").collect();
                let file_segments: Vec<&str> = path_segments.last().unwrap().split(".").collect();
                let file_name = file_segments[0];

                let data = match file_name.as_ref() {
                    "account" => csv_to_json_table::<Account>(&path, file_name, b';', row_limit),
                    "card" => csv_to_json_table::<Card>(&path, file_name, b';', row_limit),
                    "client" => csv_to_json_table::<Client>(&path, file_name, b';', row_limit),
                    "disp" => csv_to_json_table::<Disp>(&path, file_name, b';', row_limit),
                    "district" => csv_to_json_table::<District>(&path, file_name, b';', row_limit),
                    "loan" => csv_to_json_table::<Loan>(&path, file_name, b';', row_limit),
                    "order" => csv_to_json_table::<Order>(&path, file_name, b';', row_limit),
                    "trans" => csv_to_json_table::<Trans>(&path, file_name, b';', row_limit),
                    "anzsic06" => csv_to_json_table::<Business>(&path, file_name, b',', row_limit),
                    "census90" => {
                        csv_to_json_table::<CensusEntry>(&path, file_name, b',', row_limit)
                    }
                    "covid" => csv_to_json_table::<Covid>(&path, file_name, b',', row_limit),
                    _ => panic!("Unexpected table {}", file_name),
                };
                return (file_name.to_string(), data);
            })
            .map(|result| -> (String, UnitList) {
                match result.1 {
                    Err(error) => {
                        eprintln!("file error: {}", error);
                        return (String::from("error"), vec![]);
                    }
                    Ok(table) => return (result.0, table),
                }
            })
            .collect();

        for (grouping, table) in dataset.iter() {
            for (key, content) in table.iter() {
                let grouping = GroupingLabel::new(grouping.as_bytes());
                let response = client.set_unit(&grouping, &key, &content).unwrap();

                match response {
                    TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                    _ => panic!(),
                }
            }
        }

        for (grouping, table) in dataset.iter() {
            for (key, content) in table.iter() {
                let grouping = GroupingLabel::new(grouping.as_bytes());
                let response = client.get_by_key(&grouping, &key).unwrap();
                match response {
                    TcpResponse::Outcome(outcome) => {
                        assert_eq!(outcome, Outcome::Select(vec![content.to_owned()]))
                    }
                    _ => panic!(),
                }
            }
        }
    }

    #[test]
    fn tcp_e2e_remove_one() {
        let port = 8004;
        launch_test_db_servers("tcp_e2e_remove_one", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = &GroupingLabel::new("any_grouping".as_bytes());

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 3;
        let (target_key, _target_content) = &key_content_pairs[target_pair_index];

        for (key, content) in key_content_pairs.iter() {
            let response = client.set_unit(&grouping, &key, &content).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let response = client.remove_one(&grouping, &target_key).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::RemoveOneSuccess),
            _ => panic!(),
        }

        for (key, content) in key_content_pairs.iter() {
            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    if key == target_key {
                        assert_eq!(outcome, Outcome::Select(vec![]));
                    } else {
                        assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
                    }
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_revert_all() {
        let port = 8005;
        launch_test_db_servers("tcp_e2e_revert_all", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            let response = client.set_unit(&grouping, &key, &content).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let response = client.revert_all(&target_height).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::RevertAllSuccess),
            _ => panic!(),
        }

        for (index, (key, content)) in key_content_pairs.iter().enumerate() {
            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    if index <= target_height.as_u64() as usize {
                        assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
                    } else {
                        assert_eq!(outcome, Outcome::Select(vec![]));
                    }
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_remove_all() {
        let port = 8006;
        launch_test_db_servers("tcp_e2e_remove_all", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            let response = client.set_unit(&grouping, &key, &content).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let response = client.remove_all().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::RemoveAllSuccess),
            _ => panic!(),
        }

        for (key, _content) in key_content_pairs.iter() {
            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::Select(vec![])),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_atomicity_commit() {
        let port = 8007;
        launch_test_db_servers("tcp_e2e_atomicity_commit", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let response = client.create_transaction().unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(
                outcome,
                Outcome::CreateTransaction(expected_transaction_id.clone())
            ),
            _ => panic!(),
        }

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            let response = client
                .transactional_set_unit(&grouping, &key, &content, &expected_transaction_id)
                .unwrap();

            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::TransactionalInsertSuccess)
                }
                _ => panic!(),
            }
        }

        let response = client.commit_transaction(&expected_transaction_id).unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionCommitSuccess),
            _ => panic!(),
        }

        for (key, content) in key_content_pairs.iter() {
            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::Select(vec![content.clone()]))
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_atomicity_abort() {
        let port = 8008;
        launch_test_db_servers("tcp_e2e_atomicity_abort", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let response = client.create_transaction().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(
                outcome,
                Outcome::CreateTransaction(expected_transaction_id.clone())
            ),
            _ => panic!(),
        }

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            let response = client
                .transactional_set_unit(&grouping, &key, &content, &expected_transaction_id)
                .unwrap();

            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::TransactionalInsertSuccess)
                }
                _ => panic!(),
            }
        }

        let response = client.abort_transaction(&expected_transaction_id).unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionAbortSuccess),
            _ => panic!(),
        }

        for (key, _content) in key_content_pairs.iter() {
            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::Select(vec![])),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_set_isolation() {
        let port = 8009;
        launch_test_db_servers("tcp_e2e_set_isolation", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let transaction_id = TransactionId::new(1);

        let response = client.create_transaction().unwrap();

        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::CreateTransaction(transaction_id.clone()))
            }
            _ => panic!(),
        }

        let unit_key = UnitKey::from("key1");
        let content = UnitContent::String(String::from("This is string"));
        let response = client
            .transactional_set_unit(&grouping, &unit_key, &content, &transaction_id)
            .unwrap();

        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::TransactionalInsertSuccess)
            }
            _ => panic!(),
        }

        {
            let response = client
                .transactional_get(&grouping, &unit_key, &transaction_id)
                .unwrap();

            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::Select(vec![content.clone()]))
                }
                _ => panic!(),
            }
        }

        {
            let response = client.get_by_key(&grouping, &unit_key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::Select(vec![])),
                _ => panic!(),
            }
        }

        let response = client.commit_transaction(&transaction_id).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionCommitSuccess),
            _ => panic!(),
        }

        {
            let response = client.get_by_key(&grouping, &unit_key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::Select(vec![content]))
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_remove_one_isolation() {
        let port = 8010;
        launch_test_db_servers("tcp_e2e_remove_one_isolation", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let (target_key, _target_content) = &key_content_pairs[target_pair_index];

        for (key, content) in key_content_pairs.iter() {
            let response = client.set_unit(&grouping, &key, &content).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let response = client.create_transaction().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(
                outcome,
                Outcome::CreateTransaction(expected_transaction_id.clone())
            ),
            _ => panic!(),
        }

        let response = client
            .transactional_remove_one(&expected_transaction_id, &grouping, &target_key)
            .unwrap();
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::TransactionalRemoveOneSuccess)
            }
            _ => panic!(),
        }

        let response = client.commit_transaction(&expected_transaction_id).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionCommitSuccess),
            _ => panic!(),
        }

        for (key, content) in key_content_pairs.iter() {
            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    if key == target_key {
                        assert_eq!(outcome, Outcome::Select(vec![]));
                    } else {
                        assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
                    }
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_revert_one_isolation() {
        let port = 8011;
        launch_test_db_servers("tcp_e2e_revert_one_isolation", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

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
            let response = client.set_unit(&grouping, &key, &content).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let response = client.create_transaction().unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(
                outcome,
                Outcome::CreateTransaction(expected_transaction_id.clone())
            ),
            _ => panic!(),
        }

        let response = client
            .transactional_revert_one(&grouping, &key, &target_height, &expected_transaction_id)
            .unwrap();

        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::TransactionalRevertOneSuccess)
            }
            _ => panic!(),
        }

        let response = client.get_by_key(&grouping, &key).unwrap();
        let expected_output = contents.last().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::Select(vec![expected_output.clone()]))
            }
            _ => panic!(),
        }

        let response = client.commit_transaction(&expected_transaction_id).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionCommitSuccess),
            _ => panic!(),
        }

        let response = client.get_by_key(&grouping, &key).unwrap();
        let expected_content = &contents[target_pair_index as usize];
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::Select(vec![expected_content.clone()]))
            }
            _ => panic!(),
        }
    }

    #[test]
    fn tcp_e2e_transaction_not_alive_after_revert_all() {
        let port = 8014;
        launch_test_db_servers(
            "tcp_e2e_transaction_not_alive_after_revert_all",
            None,
            Some(port),
        )
        .unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            let response = client.set_unit(&grouping, &key, &content).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let response = client.create_transaction().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::CreateTransaction(expected_transaction_id))
            }
            _ => panic!(),
        }

        let response = client.revert_all(&target_height).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::RevertAllSuccess),
            _ => panic!(),
        }

        let response = client.commit_transaction(&expected_transaction_id).unwrap();
        match response {
            TcpResponse::Outcome(_outcome) => panic!(),
            TcpResponse::ServerError(error) => {
                assert_eq!(
                    error,
                    ServerError::ExecutorError(KVError(TransactionManagerError(
                        TransactionNotAlive
                    )))
                );
            }
        }
    }

    #[test]
    fn tcp_e2e_unexpected_commit_transaction_id() {
        let port = 8016;
        launch_test_db_servers("tcp_e2e_unexpected_commit_transaction_id", None, Some(port))
            .unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let fake_transaction_id = TransactionId::new(100);

        let response = client.commit_transaction(&fake_transaction_id).unwrap();
        match response {
            TcpResponse::Outcome(_) => panic!(),
            TcpResponse::ServerError(error) => assert_eq!(
                error,
                ServerError::ExecutorError(KVError(TransactionManagerError(TransactionNotAlive)))
            ),
        }
    }

    #[test]
    fn tcp_e2e_unexpected_abort_transaction_id() {
        let port = 8015;
        launch_test_db_servers("tcp_e2e_unexpected_abort_transaction_id", None, Some(port))
            .unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let fake_transaction_id = TransactionId::new(100);

        let response = client.abort_transaction(&fake_transaction_id).unwrap();
        match response {
            TcpResponse::Outcome(_) => panic!(),
            TcpResponse::ServerError(error) => assert_eq!(
                error,
                ServerError::ExecutorError(KVError(TransactionManagerError(TransactionNotAlive)))
            ),
        }
    }

    #[test]
    fn tcp_e2e_last_one_commit_wins() {
        let port = 8017;
        launch_test_db_servers("tcp_e2e_last_one_commit_wins", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let shared_keys = [UnitKey::from("a"), UnitKey::from("b"), UnitKey::from("c")];

        let key_value_pairs_1: UnitList = shared_keys
            .iter()
            .enumerate()
            .map(|(index, key)| {
                let value = format!("{:?}", index + 1);
                (key.clone(), UnitContent::String(value))
            })
            .collect();

        let key_value_pairs_2: UnitList = shared_keys
            .iter()
            .enumerate()
            .map(|(index, key)| {
                let value = format!("{:?}", (index + 1) * 100);
                (key.clone(), UnitContent::String(value))
            })
            .collect();

        let mixed_kv_pairs: Vec<_> = key_value_pairs_1.iter().zip(&key_value_pairs_2).collect();
        let expected_tid_1 = TransactionId::new(1);
        let expected_tid_2 = TransactionId::new(2);

        let response = client.create_transaction().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::CreateTransaction(expected_tid_1))
            }
            _ => panic!(),
        }

        let response = client.create_transaction().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::CreateTransaction(expected_tid_2))
            }
            _ => panic!(),
        }

        for kv_pairs in mixed_kv_pairs {
            let kv_1 = kv_pairs.0;
            let kv_2 = kv_pairs.1;

            let response = client
                .transactional_set_unit(&grouping, &kv_1.0, &kv_1.1, &expected_tid_1)
                .unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::TransactionalInsertSuccess)
                }
                _ => panic!(),
            }

            let response = client
                .transactional_set_unit(&grouping, &kv_2.0, &kv_2.1, &expected_tid_2)
                .unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::TransactionalInsertSuccess)
                }
                _ => panic!(),
            }
        }

        let response = client.commit_transaction(&expected_tid_1).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionCommitSuccess),
            _ => panic!(),
        }

        let response = client.commit_transaction(&expected_tid_2).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionCommitSuccess),
            _ => panic!(),
        }

        for (index, key) in shared_keys.iter().enumerate() {
            let response = client.get_by_key(&grouping, &key).unwrap();
            let expected_content = key_value_pairs_2[index].1.clone();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::Select(vec![expected_content]))
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_read_inside_transaction() {
        let port = 8018;
        launch_test_db_servers("tcp_e2e_read_inside_transaction", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key = UnitKey::from("a");
        let value = UnitContent::String(String::from("1"));

        let response = client.set_unit(&grouping, &key, &value).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
            _ => panic!(),
        }

        let expected_tid = TransactionId::new(1);
        let response = client.create_transaction().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::CreateTransaction(expected_tid))
            }
            _ => panic!(),
        }

        {
            let response = client
                .transactional_get(&grouping, &key, &expected_tid)
                .unwrap();
            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::Select(vec![value])),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn tcp_e2e_dirty_read() {
        let port = 8019;
        launch_test_db_servers("tcp_e2e_dirty_read", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key = UnitKey::from("a");
        let origin_value = UnitContent::String(String::from("1"));
        let value_in_transaction = UnitContent::String(String::from("2"));

        let response = client.set_unit(&grouping, &key, &origin_value).unwrap();
        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
            _ => panic!(),
        }

        let expected_tid = TransactionId::new(1);
        let response = client.create_transaction().unwrap();
        match response {
            TcpResponse::Outcome(outcome) => {
                assert_eq!(outcome, Outcome::CreateTransaction(expected_tid))
            }
            _ => panic!(),
        }

        {
            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::Select(vec![origin_value.clone()]))
                }
                _ => panic!(),
            }
        }

        {
            let response = client
                .transactional_set_unit(&grouping, &key, &value_in_transaction, &expected_tid)
                .unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    assert_eq!(outcome, Outcome::TransactionalInsertSuccess)
                }
                _ => panic!(),
            }

            let response = client.get_by_key(&grouping, &key).unwrap();
            match response {
                TcpResponse::Outcome(outcome) => {
                    let actual_outcome = Outcome::Select(vec![origin_value.clone()]);
                    assert_eq!(outcome, actual_outcome);
                }
                _ => panic!(),
            }
        }

        let response = client.commit_transaction(&expected_tid).unwrap();

        match response {
            TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::TransactionCommitSuccess),
            _ => panic!(),
        }

        {
            let response = client
                .transactional_get(&grouping, &key, &expected_tid)
                .unwrap();

            match response {
                TcpResponse::Outcome(_outcome) => panic!(),
                TcpResponse::ServerError(error) => {
                    assert_eq!(
                        error,
                        ServerError::ExecutorError(KVError(TransactionManagerError(
                            TransactionNotAlive
                        )))
                    );
                }
            }
        }
    }

    #[test]
    fn tcp_e2e_predicate_read() {
        let (expected_satisfied_contents, unsatisfied_content) = get_phone_model_fixture();

        let mut contents = vec![];
        contents.extend_from_slice(&expected_satisfied_contents);
        contents.extend_from_slice(&unsatisfied_content);

        let port = 8020;
        launch_test_db_servers("tcp_e2e_filter_read", None, Some(port)).unwrap();

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        for (index, unit_content) in contents.iter().enumerate() {
            let key_str = format!("{}", index);
            let unit_key = UnitKey::new(key_str.as_bytes());

            let response = client
                .set_unit(&grouping, &unit_key, &unit_content)
                .unwrap();

            match response {
                TcpResponse::Outcome(outcome) => assert_eq!(outcome, Outcome::InsertSuccess),
                _ => panic!(),
            }
        }

        let predicate = get_phone_mode_test_predicates();
        let response = client.get_by_predicate(&grouping, &predicate).unwrap();

        match response {
            TcpResponse::Outcome(outcome) => match outcome {
                Outcome::Select(actual_satisfied_contents) => {
                    for satisfied_content in actual_satisfied_contents.iter() {
                        assert!(expected_satisfied_contents.contains(satisfied_content));
                    }

                    for expected_satisfied_content in expected_satisfied_contents.iter() {
                        assert!(actual_satisfied_contents.contains(expected_satisfied_content));
                    }

                    assert_eq!(
                        actual_satisfied_contents.len(),
                        expected_satisfied_contents.len()
                    );
                }
                _ => panic!("Wrong outcome type."),
            },
            _ => panic!(),
        }
    }
}
