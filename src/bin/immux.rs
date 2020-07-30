use std::path::PathBuf;

use immuxsys::constants as Constants;
use immuxsys::storage::chain_height::ChainHeight;

use clap::{App, Arg, SubCommand};
use immuxsys::storage::executor::{
    errors::ExecutorResult, executor::Executor, grouping::Grouping, unit_content::UnitContent,
    unit_key::UnitKey,
};
use immuxsys::storage::transaction_manager::TransactionId;

fn main() -> ExecutorResult<()> {
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
            SubCommand::with_name(Constants::SUBCOMMAND_INSPECT_ONE)
                .about(Constants::SUBCOMMAND_INSPECT_ONE_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_KEY)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_INSPECT_ALL)
                .about(Constants::SUBCOMMAND_INSPECT_ALL_DESCRIPTION),
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
            let grouping = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let value = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_VALUE)
                .expect(Constants::MISSING_VALUE_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut executor = Executor::open(&path)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                executor.set(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
                    &UnitContent::String(value.to_string()),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                executor.set(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
                    &UnitContent::String(value.to_string()),
                    None,
                )
            }
        }
        (Constants::SUBCOMMAND_GET, Some(arg_matches)) => {
            let grouping = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
            let mut executor = Executor::open(&path)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                match executor.get(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
                    Some(TransactionId::new(transaction_id)),
                )? {
                    Some(result) => {
                        println!("{:?}", result);
                    }
                    None => {
                        println!("{:?}", Constants::MISSING_KEY_MESSAGE);
                    }
                }
            } else {
                match executor.get(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
                    None,
                )? {
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
            let grouping = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let height_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_HEIGHT)
                .expect(Constants::MISSING_HEIGHT_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut executor = Executor::open(&path)?;
            let height = height_str.parse::<u64>()?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                executor.revert_one(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
                    &ChainHeight::new(height),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                executor.revert_one(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
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

            let mut executor = Executor::open(&path)?;
            let height = height_str.parse::<u64>()?;
            executor.revert_all(&ChainHeight::new(height))
        }
        (Constants::SUBCOMMAND_REMOVE_ONE, Some(arg_matches)) => {
            let grouping = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut executor = Executor::open(&path)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                executor.remove_one(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                executor.remove_one(
                    &Grouping::new(grouping.as_bytes()),
                    &UnitKey::new(&key.as_bytes()),
                    None,
                )
            }
        }
        (Constants::SUBCOMMAND_REMOVE_ALL, Some(_arg_matches)) => {
            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut executor = Executor::open(&path)?;
            executor.remove_all()
        }
        (Constants::SUBCOMMAND_INSPECT_ONE, Some(arg_matches)) => {
            let grouping = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let unit_key = UnitKey::new(&key.as_bytes());
            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
            let mut executor = Executor::open(&path)?;

            let res = executor.inspect_one(&Grouping::new(grouping.as_bytes()), &unit_key)?;
            for command in res {
                println!("{:?}", command);
            }

            return Ok(());
        }
        (Constants::SUBCOMMAND_INSPECT_ALL, Some(_arg_matches)) => {
            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
            let mut executor = Executor::open(&path)?;
            let res = executor.inspect_all()?;
            for command in res {
                println!("{:?}", command);
            }

            return Ok(());
        }
        (Constants::SUBCOMMAND_CREATE_TRANSACTION, Some(_arg_matches)) => {
            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut executor = Executor::open(&path)?;
            let transaction_id = executor.start_transaction()?;
            println!("{:?}", transaction_id);

            return Ok(());
        }
        (Constants::SUBCOMMAND_COMMIT_TRANSACTION, Some(arg_matches)) => {
            let transaction_id_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                .expect(Constants::MISSING_TRANSACTION_ID_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut executor = Executor::open(&path)?;
            let transaction_id = transaction_id_str.parse::<u64>()?;
            executor.commit_transaction(TransactionId::new(transaction_id))
        }
        (Constants::SUBCOMMAND_ABORT_TRANSACTION, Some(arg_matches)) => {
            let transaction_id_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                .expect(Constants::MISSING_TRANSACTION_ID_ARGUMENT_MESSAGE);

            let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);

            let mut executor = Executor::open(&path)?;
            let transaction_id = transaction_id_str.parse::<u64>()?;
            executor.abort_transaction(TransactionId::new(transaction_id))
        }
        _ => unreachable!(),
    }
}
