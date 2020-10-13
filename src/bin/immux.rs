use clap::{App, Arg, SubCommand};

use immuxsys::constants as Constants;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::command::SelectCondition;
use immuxsys::storage::executor::filter::parse_filter_string;
use immuxsys::storage::executor::{
    errors::ExecutorResult, executor::Executor, grouping_label::GroupingLabel,
    unit_content::UnitContent, unit_key::UnitKey,
};
use immuxsys::storage::preferences::DBPreferences;
use immuxsys::storage::transaction_manager::TransactionId;

fn main() -> ExecutorResult<()> {
    let pref = DBPreferences::default();

    let arg_matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_SET)
                .about(Constants::SUBCOMMAND_SET_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_GROUPING)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
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
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_GROUPING)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
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
            SubCommand::with_name(Constants::SUBCOMMAND_FILTER)
                .about(Constants::SUBCOMMAND_FILTER_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_GROUPING)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_FILTER)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(Constants::SUBCOMMAND_REVERT_ONE)
                .about(Constants::SUBCOMMAND_REVERT_ONE_DESCRIPTION)
                .arg(
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_GROUPING)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
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
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_GROUPING)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
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
                    Arg::with_name(Constants::ARGUMENT_NAME_FOR_GROUPING)
                        .help(Constants::GENERAL_ARGUMENT_HELP_INFORMATION)
                        .required(true),
                )
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

    let result = match arg_matches.subcommand() {
        (Constants::SUBCOMMAND_SET, Some(arg_matches)) => {
            let grouping_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);

            let key_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let value_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_VALUE)
                .expect(Constants::MISSING_VALUE_ARGUMENT_MESSAGE);

            let mut executor = Executor::open(&pref)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                executor.set(
                    &GroupingLabel::from(grouping_arg),
                    &UnitKey::from(key_arg),
                    &UnitContent::from(value_arg),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                executor.set(
                    &GroupingLabel::from(grouping_arg),
                    &UnitKey::from(key_arg),
                    &UnitContent::from(value_arg),
                    None,
                )
            }
        }
        (Constants::SUBCOMMAND_GET, Some(arg_matches)) => {
            let grouping_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);

            let pref = DBPreferences::default_at_dir(Constants::TEMP_LOG_FILE_DIR);
            let mut executor = Executor::open(&pref)?;

            if let Some(key_arg) = arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_KEY) {
                if let Some(transaction_id_str) =
                    arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                {
                    let transaction_id = transaction_id_str.parse::<u64>()?;
                    let condition = SelectCondition::Key(
                        GroupingLabel::from(grouping_arg),
                        UnitKey::from(key_arg),
                        Some(TransactionId::new(transaction_id)),
                    );
                    executor.get(&condition)
                } else {
                    let condition = SelectCondition::Key(
                        GroupingLabel::from(grouping_arg),
                        UnitKey::from(key_arg),
                        None,
                    );
                    executor.get(&condition)
                }
            } else {
                let condition =
                    SelectCondition::UnconditionalMatch(GroupingLabel::from(grouping_arg));
                executor.get(&condition)
            }
        }
        (Constants::SUBCOMMAND_FILTER, Some(arg_matches)) => {
            let grouping_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let filter_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_FILTER)
                .expect(Constants::MISSING_FILTER_ARGUMENT_MESSAGE);
            {
                let mut executor = Executor::open(&pref)?;
                let filter = parse_filter_string(filter_str.to_string())?;

                let condition = SelectCondition::Filter(GroupingLabel::from(grouping_arg), filter);
                executor.get(&condition)
            }
        }
        (Constants::SUBCOMMAND_REVERT_ONE, Some(arg_matches)) => {
            let grouping_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let height_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_HEIGHT)
                .expect(Constants::MISSING_HEIGHT_ARGUMENT_MESSAGE);

            let mut executor = Executor::open(&pref)?;
            let height = height_arg.parse::<u64>()?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                executor.revert_one(
                    &GroupingLabel::from(grouping_arg),
                    &UnitKey::from(key_arg),
                    &ChainHeight::new(height),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                executor.revert_one(
                    &GroupingLabel::from(grouping_arg),
                    &UnitKey::from(key_arg),
                    &ChainHeight::new(height),
                    None,
                )
            }
        }
        (Constants::SUBCOMMAND_REVERT_ALL, Some(arg_matches)) => {
            let height_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_HEIGHT)
                .expect(Constants::MISSING_HEIGHT_ARGUMENT_MESSAGE);

            let mut executor = Executor::open(&pref)?;
            let height = height_arg.parse::<u64>()?;
            executor.revert_all(&ChainHeight::new(height))
        }
        (Constants::SUBCOMMAND_REMOVE_ONE, Some(arg_matches)) => {
            let grouping_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);

            let mut executor = Executor::open(&pref)?;

            if let Some(transaction_id_str) =
                arg_matches.value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
            {
                let transaction_id = transaction_id_str.parse::<u64>()?;
                executor.remove_one(
                    &GroupingLabel::from(grouping_arg),
                    &UnitKey::from(key_arg),
                    Some(TransactionId::new(transaction_id)),
                )
            } else {
                executor.remove_one(
                    &GroupingLabel::from(grouping_arg),
                    &UnitKey::from(key_arg),
                    None,
                )
            }
        }
        (Constants::SUBCOMMAND_REMOVE_ALL, Some(_arg_matches)) => {
            let mut executor = Executor::open(&pref)?;
            executor.remove_all()
        }
        (Constants::SUBCOMMAND_INSPECT_ONE, Some(arg_matches)) => {
            let grouping_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_GROUPING)
                .expect(Constants::MISSING_GROUPING_ARGUMENT_MESSAGE);
            let key_arg = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_KEY)
                .expect(Constants::MISSING_KEY_ARGUMENT_MESSAGE);
            let unit_key = UnitKey::from(key_arg);
            let mut executor = Executor::open(&pref)?;

            executor.inspect_one(&GroupingLabel::from(grouping_arg), &unit_key)
        }
        (Constants::SUBCOMMAND_INSPECT_ALL, Some(_arg_matches)) => {
            let mut executor = Executor::open(&pref)?;
            executor.inspect_all()
        }
        (Constants::SUBCOMMAND_CREATE_TRANSACTION, Some(_arg_matches)) => {
            let mut executor = Executor::open(&pref)?;
            executor.start_transaction()
        }
        (Constants::SUBCOMMAND_COMMIT_TRANSACTION, Some(arg_matches)) => {
            let transaction_id_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                .expect(Constants::MISSING_TRANSACTION_ID_ARGUMENT_MESSAGE);
            let mut executor = Executor::open(&pref)?;
            let transaction_id = transaction_id_str.parse::<u64>()?;
            executor.commit_transaction(TransactionId::new(transaction_id))
        }
        (Constants::SUBCOMMAND_ABORT_TRANSACTION, Some(arg_matches)) => {
            let transaction_id_str = arg_matches
                .value_of(Constants::ARGUMENT_NAME_FOR_TRANSACTION_ID)
                .expect(Constants::MISSING_TRANSACTION_ID_ARGUMENT_MESSAGE);

            let mut executor = Executor::open(&pref)?;
            let transaction_id = transaction_id_str.parse::<u64>()?;
            executor.abort_transaction(TransactionId::new(transaction_id))
        }
        _ => unreachable!(),
    };
    let output = result?;
    println!("{}", output);
    return Ok(());
}
