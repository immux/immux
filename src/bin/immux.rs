use std::path::PathBuf;

use immuxsys::constants as Constants;
use immuxsys::storage::chain_height::ChainHeight;

use clap::{App, Arg, SubCommand};
use immuxsys::database::database::Database;
use immuxsys::database::errors::DatabaseResult;
use immuxsys::database::unit_content::UnitContent;
use immuxsys::database::unit_key::UnitKey;
use immuxsys::storage::transaction_manager::TransactionId;

fn main() -> DatabaseResult<()> {
    let arg_matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_SET)
                .about(Constants::SUBCOMMAND_SET_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_VALUE)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_GET)
                .about(Constants::SUBCOMMAND_GET_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_REVERT_ONE)
                .about(Constants::SUBCOMMAND_REVERT_ONE_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_HEIGHT)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_REVERT_ALL)
                .about(Constants::SUBCOMMAND_REVERT_ALL_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_HEIGHT)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_REMOVE_ONE)
                .about(Constants::SUBCOMMAND_REMOVE_ONE_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_REMOVE_ALL)
                .about(Constants::SUBCOMMAND_REMOVE_ALL_DESCRIPTION),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_INSPECT)
                .about(Constants::SUBCOMMAND_INSPECT_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_CREATE_TRANSACTION)
                .about(Constants::SUBCOMMAND_CREATE_TRANSACTION_DESCRIPTION),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_COMMIT_TRANSACTION)
                .about(Constants::SUBCOMMAND_COMMIT_TRANSACTION_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_ABORT_TRANSACTION)
                .about(Constants::SUBCOMMAND_ABORT_TRANSACTION_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .get_matches();

    match arg_matches.subcommand() {
        (Constants::SUBCOMMAND_SET, Some(arg_matches)) => {
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let value = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_VALUE)
                .expect(Constants::MISSING_VALUE_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                store_engine.set(
                    UnitKey::new(&key.as_bytes()),
                    UnitContent::String(value.to_string()),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                store_engine.set(
                    UnitKey::new(&key.as_bytes()),
                    UnitContent::String(value.to_string()),
                    None,
                )
            }
        }
        (Constants::SUBCOMMAND_GET, Some(arg_matches)) => {
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
            let mut store_engine = Database::open(&path)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                match store_engine.get(
                    &UnitKey::new(&key.as_bytes()),
                    &Some(TransactionId::new(transaction_id)),
                )? {
                    Some(result) => {
                        println!("{:?}", result);
                    }
                    None => {
                        println!("{:?}", Constants::MISSING_KEY_MESSAGE);
                    }
                }
            } else {
                match store_engine.get(&UnitKey::new(&key.as_bytes()), &None)? {
                    Some(result) => {
                        println!("{:?}", result);
                    }
                    None => {
                        println!("{:?}", Constants::MISSING_KEY_MESSAGE);
                    }
                }
            }

            return Ok(());
        }
        (Constants::SUBCOMMAND_REVERT_ONE, Some(arg_matches)) => {
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let height_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_HEIGHT)
                .expect(Constants::MISSING_HEIGHT_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;
            let height = height_str.parse::<u64>()?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                store_engine.revert_one(
                    UnitKey::new(&key.as_bytes()),
                    &ChainHeight::new(height),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                store_engine.revert_one(
                    UnitKey::new(&key.as_bytes()),
                    &ChainHeight::new(height),
                    None,
                )
            }
        }
        (Constants::SUBCOMMAND_REVERT_ALL, Some(arg_matches)) => {
            let height_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_HEIGHT)
                .expect(Constants::MISSING_HEIGHT_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;
            let height = height_str.parse::<u64>()?;
            store_engine.revert_all(&ChainHeight::new(height))
        }
        (Constants::SUBCOMMAND_REMOVE_ONE, Some(arg_matches)) => {
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                store_engine.remove_one(
                    UnitKey::new(&key.as_bytes()),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                store_engine.remove_one(UnitKey::new(&key.as_bytes()), None)
            }
        }
        (Constants::SUBCOMMAND_REMOVE_ALL, Some(_arg_matches)) => {
            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;
            store_engine.remove_all()
        }
        (Constants::SUBCOMMAND_INSPECT, Some(arg_matches)) => {
            let key = arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_KEY);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;

            match key {
                None => {
                    let res = store_engine.inspect(None)?;
                    for command in res {
                        println!("{:?}", command);
                    }
                }
                Some(key_str) => {
                    let key_bytes = key_str.as_bytes().to_vec();
                    let kvkey = UnitKey::new(&key_bytes);
                    let res = store_engine.inspect(Some(&kvkey))?;
                    for command in res {
                        println!("{:?}", command);
                    }
                }
            }

            return Ok(());
        }
        (Constants::SUBCOMMAND_CREATE_TRANSACTION, Some(_arg_matches)) => {
            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;
            let transaction_id = store_engine.start_transaction()?;
            println!("{:?}", transaction_id);

            return Ok(());
        }
        (Constants::SUBCOMMAND_COMMIT_TRANSACTION, Some(arg_matches)) => {
            let transaction_id_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                .expect(Constants::MISSING_TRANSACTION_ID_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;
            let transaction_id = transaction_id_str.parse::<u64>()?;
            store_engine.commit_transaction(TransactionId::new(transaction_id))
        }
        (Constants::SUBCOMMAND_ABORT_TRANSACTION, Some(arg_matches)) => {
            let transaction_id_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                .expect(Constants::MISSING_TRANSACTION_ID_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut store_engine = Database::open(&path)?;
            let transaction_id = transaction_id_str.parse::<u64>()?;
            store_engine.abort_transaction(TransactionId::new(transaction_id))
        }
        _ => unreachable!(),
    }
}
