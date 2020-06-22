pub const COMMAND_VERSION_FLAGS: &str = "-V";

pub const SUBCOMMAND_GET: &str = "get";
pub const SUBCOMMAND_SET: &str = "set";
pub const SUBCOMMAND_REVERT_ONE: &str = "revert_one";
pub const SUBCOMMAND_REVERT_ALL: &str = "revert_all";
pub const SUBCOMMAND_REMOVE_ONE: &str = "remove_one";
pub const SUBCOMMAND_REMOVE_ALL: &str = "remove_all";
pub const SUBCOMMAND_INSPECT: &str = "inspect";

pub const SUBCOMMAND_SET_DESCRIPTION: &str = "Set the string key value pair.";
pub const SUBCOMMAND_GET_DESCRIPTION: &str = "Get the string value of a given string key.";
pub const SUBCOMMAND_REVERT_ONE_DESCRIPTION: &str =
    "Revert the string value of a given string key.";
pub const SUBCOMMAND_REVERT_ALL_DESCRIPTION: &str = "Revert the whole db.";
pub const SUBCOMMAND_REMOVE_ONE_DESCRIPTION: &str = "Remove the key";
pub const SUBCOMMAND_REMOVE_ALL_DESCRIPTION: &str = "Clear the whole db";
pub const SUBCOMMAND_INSPECT_DESCRIPTION: &str = "Inspect the string value of a given string key.";

pub const GENERAL_ARGUMENT_HELP_INFORMATION: &str = "A string key";

pub const ARGUMENT_NAME_FOR_KEY: &str = "KEY";
pub const MISSING_KEY_ARGUMENT_MESSAGE: &str = "KEY argument missing";
pub const ARGUMENT_NAME_FOR_VALUE: &str = "VALUE";
pub const MISSING_VALUE_ARGUMENT_MESSAGE: &str = "VALUE argument missing";
pub const ARGUMENT_NAME_FOR_HEIGHT: &str = "HEIGHT";
pub const MISSING_HEIGHT_ARGUMENT_MESSAGE: &str = "HEIGHT argument missing";
pub const MISSING_KEY_MESSAGE: &str = "Key not existed";

pub const TEMP_LOG_FILE_PATH: &str = "/tmp";

pub const LOG_FILE_NAME: &str = "command_log";

pub const MAX_KEY_LENGTH: usize = 8 * 1024;
