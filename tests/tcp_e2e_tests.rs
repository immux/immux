#[cfg(test)]
mod tcp_e2e_tests {
    use std::collections::HashMap;
    use std::error::Error;

    use immuxsys::constants as Constants;
    use immuxsys::storage::chain_height::ChainHeight;
    use immuxsys::storage::executor::command::{Command, SelectCondition};
    use immuxsys::storage::executor::grouping_label::GroupingLabel;
    use immuxsys::storage::executor::outcome::Outcome;
    use immuxsys::storage::executor::unit_content::UnitContent;
    use immuxsys::storage::executor::unit_key::UnitKey;
    use immuxsys::storage::transaction_manager::TransactionId;
    use immuxsys_client::tcp_client::ImmuxDBTcpClient;
    use immuxsys_dev_utils::data_models::berka99::{
        Account, Card, Client, Disp, District, Loan, Order, Trans,
    };
    use immuxsys_dev_utils::data_models::business::Business;
    use immuxsys_dev_utils::data_models::census90::CensusEntry;
    use immuxsys_dev_utils::data_models::covid::Covid;
    use immuxsys_dev_utils::dev_utils::{
        csv_to_json_table, get_filter, get_key_content_pairs, launch_db_server, notified_sleep,
        UnitList,
    };

    #[test]
    fn tcp_e2e_grouping_get_set() {
        let port = 8000;
        launch_db_server("tcp_e2e_grouping_get_set", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let unit_key = UnitKey::new("key".as_bytes());
        let content = UnitContent::String("content".to_string());

        let command = Command::Insert {
            grouping,
            key: unit_key.clone(),
            content,
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::InsertSuccess);

        let grouping = GroupingLabel::new("the_other_grouping".as_bytes());
        let condition = SelectCondition::Key(unit_key, None);
        let command = Command::Select {
            grouping,
            condition,
        };

        client.write(&command.marshal()).unwrap();
        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::Select(vec![]));
    }

    #[test]
    fn tcp_e2e_single_unit_get_set() {
        let port = 8001;
        launch_db_server("tcp_e2e_single_unit_get_set", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = GroupingLabel::new("a".as_bytes());
        let unit_key = UnitKey::new("key".as_bytes());
        let unit_content = UnitContent::String("content".to_string());
        let command = Command::Insert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
        };

        client.write(&command.marshal()).unwrap();
        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::InsertSuccess);

        let condition = SelectCondition::Key(unit_key, None);
        let command = Command::Select {
            grouping,
            condition,
        };

        client.write(&command.marshal()).unwrap();
        let buffer = client.read().unwrap();
        let (actual_output, _) = Outcome::parse(&buffer).unwrap();
        let expected_output = Outcome::Select(vec![unit_content]);

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn tcp_e2e_revert_one() {
        let port = 8002;
        launch_db_server("tcp_e2e_revert_one", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = String::from("any_grouping");
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
            let command = Command::Insert {
                grouping: GroupingLabel::new(&grouping.as_bytes()),
                key: unit_key.clone(),
                content: content.clone(),
            };
            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let revert_command = Command::RevertOne {
            grouping: GroupingLabel::new(&grouping.as_bytes()),
            key: unit_key.clone(),
            height: target_height.clone(),
        };

        client.write(&revert_command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();
        assert_eq!(outcome, Outcome::RevertOneSuccess);

        let condition = SelectCondition::Key(unit_key, None);
        let select_command = Command::Select {
            grouping: GroupingLabel::new(&grouping.as_bytes()),
            condition,
        };

        client.write(&select_command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (actual_outcome, _) = Outcome::parse(&buffer).unwrap();

        let expected_content = &contents[target_height.as_u64() as usize];
        assert_eq!(
            actual_outcome,
            Outcome::Select(vec![expected_content.clone()])
        );
    }

    #[test]
    fn tcp_e2e_real_data_get_set() {
        let port = 8003;
        launch_db_server("tcp_e2e_real_data_get_set", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();

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
                let command = Command::Insert {
                    grouping: GroupingLabel::new(&grouping.as_bytes()),
                    key: key.to_owned(),
                    content: content.to_owned(),
                };
                client.write(&command.marshal()).unwrap();
                let buffer = client.read().unwrap();
                let (outcome, _) = Outcome::parse(&buffer).unwrap();

                assert_eq!(outcome, Outcome::InsertSuccess);
            }
        }

        for (grouping, table) in dataset.iter() {
            for (key, content) in table.iter() {
                let condition = SelectCondition::Key(key.to_owned(), None);
                let command = Command::Select {
                    grouping: GroupingLabel::new(&grouping.as_bytes()),
                    condition,
                };
                client.write(&command.marshal()).unwrap();
                let buffer = client.read().unwrap();
                let (outcome, _) = Outcome::parse(&buffer).unwrap();
                assert_eq!(outcome, Outcome::Select(vec![content.to_owned()]));
            }
        }
    }

    #[test]
    fn tcp_e2e_remove_one() {
        let port = 8004;
        launch_db_server("tcp_e2e_remove_one", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = &GroupingLabel::new("any_grouping".as_bytes());

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 3;
        let (target_key, _target_content) = &key_content_pairs[target_pair_index];

        for (key, content) in key_content_pairs.iter() {
            let command = Command::Insert {
                grouping: grouping.to_owned(),
                key: key.to_owned(),
                content: content.to_owned(),
            };
            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::RemoveOne {
            grouping: grouping.clone(),
            key: target_key.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();
        assert_eq!(outcome, Outcome::RemoveOneSuccess);

        for (key, content) in key_content_pairs.iter() {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };
            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            if key == target_key {
                assert_eq!(outcome, Outcome::Select(vec![]));
            } else {
                assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
            }
        }
    }

    #[test]
    fn tcp_e2e_revert_all() {
        let port = 8005;
        launch_db_server("tcp_e2e_revert_all", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            let command = Command::Insert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
            };
            client.write(&command.marshal()).unwrap();
            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::RevertAll {
            height: target_height.clone(),
        };
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::RevertAllSuccess);

        for (index, (key, content)) in key_content_pairs.iter().enumerate() {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };
            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            if index <= target_height.as_u64() as usize {
                assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
            } else {
                assert_eq!(outcome, Outcome::Select(vec![]));
            }
        }
    }

    #[test]
    fn tcp_e2e_remove_all() {
        let port = 8006;
        launch_db_server("tcp_e2e_remove_all", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            let command = Command::Insert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::RemoveAll;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::RemoveAllSuccess);

        for (key, _content) in key_content_pairs.iter() {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };
            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![]));
        }
    }

    #[test]
    fn tcp_e2e_atomicity_commit() {
        let port = 8007;
        launch_db_server("tcp_e2e_atomicity_commit", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(
            outcome,
            Outcome::CreateTransaction(expected_transaction_id.clone())
        );

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            let command = Command::TransactionalInsert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
                transaction_id: expected_transaction_id,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::TransactionalInsertSuccess);
        }

        let command = Command::TransactionCommit {
            transaction_id: expected_transaction_id,
        };
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionCommitSuccess);

        for (key, content) in key_content_pairs.iter() {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();
            assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
        }
    }

    #[test]
    fn tcp_e2e_atomicity_abort() {
        let port = 8008;
        launch_db_server("tcp_e2e_atomicity_abort", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(
            outcome,
            Outcome::CreateTransaction(expected_transaction_id.clone())
        );

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            let command = Command::TransactionalInsert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
                transaction_id: expected_transaction_id,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::TransactionalInsertSuccess);
        }

        let command = Command::TransactionAbort {
            transaction_id: expected_transaction_id,
        };
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionAbortSuccess);

        for (key, _content) in key_content_pairs.iter() {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![]));
        }
    }

    #[test]
    fn tcp_e2e_set_isolation() {
        let port = 8009;
        launch_db_server("tcp_e2e_set_isolation", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let transaction_id = TransactionId::new(1);

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::CreateTransaction(transaction_id.clone()));

        let unit_key = UnitKey::from("key1");
        let content = UnitContent::String(String::from("This is string"));
        let command = Command::TransactionalInsert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: content.clone(),
            transaction_id: transaction_id.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionalInsertSuccess);

        {
            let condition = SelectCondition::Key(unit_key.clone(), Some(transaction_id.clone()));
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };
            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
        }

        {
            let condition = SelectCondition::Key(unit_key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![]));
        }

        let command = Command::TransactionCommit {
            transaction_id: transaction_id.clone(),
        };
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionCommitSuccess);

        {
            let condition = SelectCondition::Key(unit_key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![content]));
        }
    }

    #[test]
    fn tcp_e2e_remove_one_isolation() {
        let port = 8010;
        launch_db_server("tcp_e2e_remove_one_isolation", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let (target_key, _target_content) = &key_content_pairs[target_pair_index];

        for (key, content) in key_content_pairs.iter() {
            let command = Command::Insert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(
            outcome,
            Outcome::CreateTransaction(expected_transaction_id.clone())
        );

        let command = Command::TransactionalRemoveOne {
            grouping: grouping.clone(),
            key: target_key.clone(),
            transaction_id: expected_transaction_id,
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionalRemoveOneSuccess);

        let command = Command::TransactionCommit {
            transaction_id: expected_transaction_id.clone(),
        };
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionCommitSuccess);

        for (key, content) in key_content_pairs.iter() {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            if key == target_key {
                assert_eq!(outcome, Outcome::Select(vec![]));
            } else {
                assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
            }
        }
    }

    #[test]
    fn tcp_e2e_revert_one_isolation() {
        let port = 8011;
        launch_db_server("tcp_e2e_revert_one_isolation", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
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
            let command = Command::Insert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(
            outcome,
            Outcome::CreateTransaction(expected_transaction_id.clone())
        );

        let command = Command::TransactionalRevertOne {
            grouping: grouping.clone(),
            key: key.clone(),
            height: target_height.clone(),
            transaction_id: expected_transaction_id.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionalRevertOneSuccess);

        let condition = SelectCondition::Key(key.clone(), None);
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        let expected_output = contents.last().unwrap();
        assert_eq!(outcome, Outcome::Select(vec![expected_output.clone()]));

        let command = Command::TransactionCommit {
            transaction_id: expected_transaction_id.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionCommitSuccess);

        let condition = SelectCondition::Key(key.clone(), None);
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        let expected_content = &contents[target_pair_index as usize];
        assert_eq!(outcome, Outcome::Select(vec![expected_content.clone()]));
    }

    #[test]
    fn tcp_e2e_remove_all_isolation() {
        let port = 8012;
        launch_db_server("tcp_e2e_remove_all_isolation", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let key_content_pairs = get_key_content_pairs();

        for (key, content) in key_content_pairs.iter() {
            let command = Command::Insert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(
            outcome,
            Outcome::CreateTransaction(expected_transaction_id.clone())
        );

        let command = Command::RemoveAll;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::RemoveAllSuccess);

        for (key, _content) in key_content_pairs.iter() {
            let condition = SelectCondition::Key(key.clone(), Some(expected_transaction_id));
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };
            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![]));
        }
    }

    #[test]
    fn tcp_e2e_revert_all_isolation() {
        let port = 8013;
        launch_db_server("tcp_e2e_revert_all_isolation", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 4;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            let command = Command::Insert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::CreateTransaction(expected_transaction_id));

        let command = Command::RevertAll {
            height: target_height.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::RevertAllSuccess);

        for (index, (key, content)) in key_content_pairs.iter().enumerate() {
            let condition = SelectCondition::Key(key.clone(), Some(expected_transaction_id));
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            if index <= target_height.as_u64() as usize {
                assert_eq!(outcome, Outcome::Select(vec![content.clone()]));
            } else {
                assert_eq!(outcome, Outcome::Select(vec![]));
            }
        }
    }

    #[test]
    #[should_panic]
    fn tcp_e2e_transaction_not_alive_after_revert_all() {
        let port = 8014;
        let handlers = launch_db_server(
            "tcp_e2e_transaction_not_alive_after_revert_all",
            None,
            Some(port),
        )
        .unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());
        let expected_transaction_id = TransactionId::new(1);

        let key_content_pairs = get_key_content_pairs();
        let target_pair_index = 2;
        let target_height = ChainHeight::new(target_pair_index);

        for (key, content) in key_content_pairs.iter() {
            let command = Command::Insert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: content.clone(),
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::CreateTransaction(expected_transaction_id));

        let command = Command::RevertAll {
            height: target_height.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::RevertAllSuccess);

        let command = Command::TransactionCommit {
            transaction_id: expected_transaction_id,
        };

        client.write(&command.marshal()).unwrap();

        for handler in handlers {
            let server_result = handler.join().unwrap();
            server_result.unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn tcp_e2e_unexpected_commit_transaction_id() {
        let port = 8016;
        let handlers =
            launch_db_server("tcp_e2e_unexpected_commit_transaction_id", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();

        let fake_transaction_id = TransactionId::new(100);

        let command = Command::TransactionCommit {
            transaction_id: fake_transaction_id,
        };

        client.write(&command.marshal()).unwrap();
        for handler in handlers {
            let server_result = handler.join().unwrap();
            server_result.unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn tcp_e2e_unexpected_abort_transaction_id() {
        let port = 8015;
        let handlers =
            launch_db_server("tcp_e2e_unexpected_abort_transaction_id", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();

        let fake_transaction_id = TransactionId::new(100);

        let command = Command::TransactionAbort {
            transaction_id: fake_transaction_id,
        };

        client.write(&command.marshal()).unwrap();
        for handler in handlers {
            let server_result = handler.join().unwrap();
            server_result.unwrap();
        }
    }

    #[test]
    fn tcp_e2e_last_one_commit_wins() {
        let port = 8017;
        launch_db_server("tcp_e2e_last_one_commit_wins", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
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

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::CreateTransaction(expected_tid_1));

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::CreateTransaction(expected_tid_2));

        for kv_pairs in mixed_kv_pairs {
            let kv_1 = kv_pairs.0;
            let kv_2 = kv_pairs.1;

            let command = Command::TransactionalInsert {
                grouping: grouping.clone(),
                key: kv_1.0.clone(),
                content: kv_1.1.clone(),
                transaction_id: expected_tid_1,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::TransactionalInsertSuccess);

            let command = Command::TransactionalInsert {
                grouping: grouping.clone(),
                key: kv_2.0.clone(),
                content: kv_2.1.clone(),
                transaction_id: expected_tid_2,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::TransactionalInsertSuccess);
        }

        let command = Command::TransactionCommit {
            transaction_id: expected_tid_1,
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionCommitSuccess);

        let command = Command::TransactionCommit {
            transaction_id: expected_tid_2,
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionCommitSuccess);

        for (index, key) in shared_keys.iter().enumerate() {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            let expected_content = key_value_pairs_2[index].1.clone();
            assert_eq!(outcome, Outcome::Select(vec![expected_content]));
        }
    }

    #[test]
    fn tcp_e2e_read_inside_transaction() {
        let port = 8018;
        launch_db_server("tcp_e2e_read_inside_transaction", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key = UnitKey::from("a");
        let value = UnitContent::String(String::from("1"));

        let command = Command::Insert {
            grouping: grouping.clone(),
            key: key.clone(),
            content: value.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::InsertSuccess);

        let expected_tid = TransactionId::new(1);
        let command = Command::CreateTransaction;

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::CreateTransaction(expected_tid));

        {
            let condition = SelectCondition::Key(key.clone(), Some(expected_tid));
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![value]));
        }
    }

    #[test]
    fn tcp_e2e_dirty_read() {
        let port = 8019;
        launch_db_server("tcp_e2e_dirty_read", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();
        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        let key = UnitKey::from("a");
        let origin_value = UnitContent::String(String::from("1"));
        let value_in_transaction = UnitContent::String(String::from("2"));

        let command = Command::Insert {
            grouping: grouping.clone(),
            key: key.clone(),
            content: origin_value.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::InsertSuccess);

        let expected_tid = TransactionId::new(1);

        let command = Command::CreateTransaction;
        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::CreateTransaction(expected_tid));

        {
            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![origin_value.clone()]));
        }

        {
            let command = Command::TransactionalInsert {
                grouping: grouping.clone(),
                key: key.clone(),
                content: value_in_transaction.clone(),
                transaction_id: expected_tid,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::TransactionalInsertSuccess);

            let condition = SelectCondition::Key(key.clone(), None);
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![origin_value.clone()]));
        }

        let command = Command::TransactionCommit {
            transaction_id: expected_tid.clone(),
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        assert_eq!(outcome, Outcome::TransactionCommitSuccess);

        {
            let condition = SelectCondition::Key(key.clone(), Some(expected_tid));
            let command = Command::Select {
                grouping: grouping.clone(),
                condition,
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::Select(vec![value_in_transaction]));
        }
    }

    #[test]
    fn tcp_e2e_filter_read() {
        let expected_satisfied_contents = vec![
            {
                let mut map = HashMap::new();
                map.insert(
                    String::from("brand"),
                    UnitContent::String(String::from("Apple")),
                );
                map.insert(String::from("price"), UnitContent::Float64(3000.0));
                map.insert(String::from("used"), UnitContent::Bool(true));
                map.insert(String::from("size"), UnitContent::Float64(13.0));

                UnitContent::Map(map)
            },
            {
                let mut map = HashMap::new();
                map.insert(
                    String::from("brand"),
                    UnitContent::String(String::from("Microsoft")),
                );
                map.insert(String::from("price"), UnitContent::Float64(1300.0));
                map.insert(String::from("used"), UnitContent::Bool(false));
                map.insert(String::from("size"), UnitContent::Float64(13.0));

                UnitContent::Map(map)
            },
            {
                let mut map = HashMap::new();
                map.insert(
                    String::from("brand"),
                    UnitContent::String(String::from("IBM")),
                );
                map.insert(String::from("price"), UnitContent::Float64(1900.0));
                map.insert(String::from("used"), UnitContent::Bool(true));
                map.insert(String::from("size"), UnitContent::Float64(11.0));

                UnitContent::Map(map)
            },
            {
                let mut map = HashMap::new();
                map.insert(
                    String::from("brand"),
                    UnitContent::String(String::from("Apple")),
                );
                map.insert(String::from("price"), UnitContent::Float64(1500.0));
                map.insert(String::from("used"), UnitContent::Bool(false));
                map.insert(String::from("size"), UnitContent::Float64(9.0));

                UnitContent::Map(map)
            },
        ];

        let unsatisfied_content = vec![
            {
                let mut map = HashMap::new();
                map.insert(
                    String::from("brand"),
                    UnitContent::String(String::from("Apple")),
                );
                map.insert(String::from("price"), UnitContent::Float64(500.0));
                map.insert(String::from("used"), UnitContent::Bool(true));
                map.insert(String::from("size"), UnitContent::Float64(13.0));

                UnitContent::Map(map)
            },
            {
                let mut map = HashMap::new();
                map.insert(
                    String::from("brand"),
                    UnitContent::String(String::from("Apple")),
                );
                map.insert(String::from("price"), UnitContent::Float64(1500.0));
                map.insert(String::from("used"), UnitContent::Bool(true));
                map.insert(String::from("size"), UnitContent::Float64(15.0));

                UnitContent::Map(map)
            },
        ];

        let mut contents = vec![];
        contents.extend_from_slice(&expected_satisfied_contents);
        contents.extend_from_slice(&unsatisfied_content);

        let port = 8020;
        launch_db_server("tcp_e2e_filter_read", None, Some(port)).unwrap();
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let mut client = ImmuxDBTcpClient::new(host).unwrap();

        let grouping = GroupingLabel::new("any_grouping".as_bytes());

        for (index, unit_content) in contents.iter().enumerate() {
            let key_str = format!("{}", index);
            let unit_key = UnitKey::new(key_str.as_bytes());

            let command = Command::Insert {
                grouping: grouping.clone(),
                key: unit_key.clone(),
                content: unit_content.clone(),
            };

            client.write(&command.marshal()).unwrap();

            let buffer = client.read().unwrap();
            let (outcome, _) = Outcome::parse(&buffer).unwrap();

            assert_eq!(outcome, Outcome::InsertSuccess);
        }

        let filter = get_filter();

        let condition = SelectCondition::Filter(filter);
        let command = Command::Select {
            grouping: grouping.clone(),
            condition,
        };

        client.write(&command.marshal()).unwrap();

        let buffer = client.read().unwrap();
        let (outcome, _) = Outcome::parse(&buffer).unwrap();

        match outcome {
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
        }
    }
}
