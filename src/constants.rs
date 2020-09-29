pub const COMMAND_VERSION_FLAGS: &str = "-V";

pub const SUBCOMMAND_GET: &str = "get";
pub const SUBCOMMAND_SET: &str = "set";
pub const SUBCOMMAND_REVERT_ONE: &str = "revert_one";
pub const SUBCOMMAND_REVERT_ALL: &str = "revert_all";
pub const SUBCOMMAND_REMOVE_ONE: &str = "remove_one";
pub const SUBCOMMAND_REMOVE_ALL: &str = "remove_all";
pub const SUBCOMMAND_INSPECT_ONE: &str = "inspect_one";
pub const SUBCOMMAND_INSPECT_ALL: &str = "inspect_all";
pub const SUBCOMMAND_CREATE_TRANSACTION: &str = "create_transaction";
pub const SUBCOMMAND_COMMIT_TRANSACTION: &str = "commit_transaction";
pub const SUBCOMMAND_ABORT_TRANSACTION: &str = "abort_transaction";
pub const SUBCOMMAND_FILTER: &str = "filter";

pub const SUBCOMMAND_SET_DESCRIPTION: &str = "Set the string key value pair.";
pub const SUBCOMMAND_GET_DESCRIPTION: &str = "Get the string value of a given string key.";
pub const SUBCOMMAND_REVERT_ONE_DESCRIPTION: &str =
    "Revert the string value of a given string key.";
pub const SUBCOMMAND_REVERT_ALL_DESCRIPTION: &str = "Revert the whole db.";
pub const SUBCOMMAND_REMOVE_ONE_DESCRIPTION: &str = "Remove the key";
pub const SUBCOMMAND_REMOVE_ALL_DESCRIPTION: &str = "Clear the whole db";
pub const SUBCOMMAND_INSPECT_ONE_DESCRIPTION: &str =
    "Inspect the string value of a given string key.";
pub const SUBCOMMAND_INSPECT_ALL_DESCRIPTION: &str = "Inspect the whole log.";
pub const SUBCOMMAND_CREATE_TRANSACTION_DESCRIPTION: &str = "Create transaction";
pub const SUBCOMMAND_COMMIT_TRANSACTION_DESCRIPTION: &str = "Commit transaction";
pub const SUBCOMMAND_ABORT_TRANSACTION_DESCRIPTION: &str = "Abort transaction";
pub const SUBCOMMAND_FILTER_DESCRIPTION: &str = "Filter out result";

pub const GENERAL_ARGUMENT_HELP_INFORMATION: &str = "A string key";

pub const ARGUMENT_NAME_FOR_GROUPING: &str = "GROUPING";
pub const MISSING_GROUPING_ARGUMENT_MESSAGE: &str = "GROUPING argument missing";
pub const ARGUMENT_NAME_FOR_KEY: &str = "KEY";
pub const MISSING_KEY_ARGUMENT_MESSAGE: &str = "KEY argument missing";
pub const ARGUMENT_NAME_FOR_VALUE: &str = "VALUE";
pub const MISSING_VALUE_ARGUMENT_MESSAGE: &str = "VALUE argument missing";
pub const ARGUMENT_NAME_FOR_HEIGHT: &str = "HEIGHT";
pub const MISSING_HEIGHT_ARGUMENT_MESSAGE: &str = "HEIGHT argument missing";
pub const ARGUMENT_NAME_FOR_TRANSACTION_ID: &str = "TRANSACTION_ID";
pub const MISSING_TRANSACTION_ID_ARGUMENT_MESSAGE: &str = "TRANSACTION_ID argument missing";
pub const ARGUMENT_NAME_FOR_FILTER: &str = "FILTER";
pub const MISSING_FILTER_ARGUMENT_MESSAGE: &str = "FILTER argument missing";
pub const MISSING_KEY_MESSAGE: &str = "Key not existed";
pub const MISSING_FILTER_MESSAGE: &str = "FILTER argument missing";

pub const TEMP_LOG_FILE_PATH: &str = "/tmp";
pub const DEFAULT_LOG_FILE_PATH: &str = "/var/immux/";
pub const LOG_FILE_NAME: &str = "command_log";

pub const MAX_KEY_LENGTH: usize = 8 * 1024;

pub const MAX_TRANSACTION_ID: u64 = u64::MAX;

pub const SERVER_END_POINT: &str = "127.0.0.1";
pub const HTTP_SERVER_DEFAULT_PORT: u16 = 6324;
pub const TCP_SERVER_DEFAULT_PORT: u16 = 5213;
pub const HEIGHT: &str = "height";
pub const REVERT_ALL_KEYWORD: &str = "revert_all";
pub const COMMIT_TRANSACTION_KEY_WORD: &str = "commit";
pub const ABORT_TRANSACTION_KEY_WORD: &str = "abort";

pub const URL_TRANSACTIONS_KEY_WORD: &str = ".transactions";
pub const URL_JOURNAL_KEY_WORD: &str = ".journal";

pub const FILTER_KEY_WORD: &str = "filter";
pub const FILTER_GREATER_OR_EQUAL: &str = ">=";
pub const FILTER_LESS_OR_EQUAL: &str = "<=";
pub const FILTER_NOT_EQUAL: &str = "!=";
pub const FILTER_LESS: &str = "<";
pub const FILTER_GREATER: &str = ">";
pub const FILTER_EQUAL: &str = "==";
