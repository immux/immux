use std::collections::HashMap;
use std::error::Error;
use std::fs::read;
use std::fs::{create_dir_all, remove_dir_all};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;
use std::{io, thread};

pub use serde::de::{Deserialize, DeserializeOwned};
pub use serde::ser::Serialize;

use immuxsys::server::errors::ServerResult;
use immuxsys::server::server::run_db_servers;
pub use immuxsys::storage::chain_height::ChainHeightError;
pub use immuxsys::storage::ecc::ErrorCorrectionError;
pub use immuxsys::storage::errors::KVError;
pub use immuxsys::storage::executor::command::{CommandError, SelectConditionError};
pub use immuxsys::storage::executor::errors::ExecutorError;
pub use immuxsys::storage::executor::grouping_label::{GroupingLabel, GroupingLabelError};
pub use immuxsys::storage::executor::predicate::{
    CompoundPredicate, FieldPath, Predicate, PredicateError, PrimitivePredicate,
};
pub use immuxsys::storage::executor::unit_content::{UnitContent, UnitContentError};
pub use immuxsys::storage::executor::unit_key::{UnitKey, UnitKeyError};
pub use immuxsys::storage::instruction::InstructionError;
pub use immuxsys::storage::kvkey::KVKeyError;
pub use immuxsys::storage::log_version::LogVersionError;
use immuxsys::storage::preferences::DBPreferences;
pub use immuxsys::storage::transaction_manager::TransactionManagerError;
use immuxsys::system_error::SystemError;
pub use immuxsys::utils::varint::VarIntError;

use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_client::ImmuxDBClient;

pub type UnitList = Vec<(UnitKey, UnitContent)>;

pub fn reset_db_dir(path: &str) -> io::Result<()> {
    println!("Initializing database in {}", path);
    create_dir_all(&path)?;
    remove_dir_all(&path)?;
    println!("Existing test data removed");
    return Ok(());
}

pub fn launch_test_db_servers(
    project_name: &str,
    http_port: Option<u16>,
    tcp_port: Option<u16>,
) -> ServerResult<()> {
    let data_root = format!("/tmp/{}/", project_name);
    reset_db_dir(&data_root)?;

    let args: Vec<String> = std::env::args().collect();

    let mut preferences = DBPreferences::from_cli_args(&args);
    preferences.log_dir = PathBuf::from(data_root);
    preferences.http_port = http_port;
    preferences.tcp_port = tcp_port;

    thread::spawn(move || run_db_servers(&preferences));
    notified_sleep(5);

    return Ok(());
}

fn notified_sleep(sec: u16) -> () {
    println!("Waiting {}s...", sec);
    thread::sleep(Duration::from_secs(sec as u64));
}

pub fn read_usize_from_arguments(position: usize) -> Result<usize, ParseIntError> {
    std::env::args()
        .nth(position)
        .unwrap_or(String::new())
        .parse::<usize>()
}

pub fn csv_to_json_table_with_size<J: DeserializeOwned + Serialize>(
    csv_file_path: &str,
    table_name: &str,
    delimiter: u8,
    bytes_limit: usize,
) -> Result<(UnitList, usize), Box<dyn Error>> {
    let reading = read(csv_file_path)?;
    let mut csv_parsing = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(reading.as_slice());
    let mut read_bytes: usize = 0;
    let mut data: UnitList = vec![];

    for (index, result) in csv_parsing.records().enumerate() {
        if read_bytes > bytes_limit {
            break;
        }
        match result {
            Ok(string_record) => {
                let journal: J = string_record.deserialize(None)?;
                let string = serde_json::to_string(&journal)?;
                let mut unit_key_str = table_name.to_string();
                unit_key_str.push_str(&index.to_string());
                let unit_key = UnitKey::from(unit_key_str.as_str());
                let content = UnitContent::from(string.as_str());

                read_bytes += unit_key.as_bytes().len();
                read_bytes += content.marshal().len();

                data.push((unit_key, content));
            }
            Err(_err) => {
                let mut unit_key_str = table_name.to_string();
                unit_key_str.push_str(&index.to_string());
                let unit_key = UnitKey::from(unit_key_str.as_str());
                let content = UnitContent::String(String::from("json:error"));

                read_bytes += unit_key.as_bytes().len();
                read_bytes += content.marshal().len();

                data.push((unit_key, content));
            }
        }
    }

    return Ok((data, read_bytes));
}

pub fn csv_to_json_table<J: DeserializeOwned + Serialize>(
    csv_file_path: &str,
    table_name: &str,
    delimiter: u8,
    row_limit: usize,
) -> Result<UnitList, Box<dyn Error>> {
    let reading = read(csv_file_path)?;
    let mut csv_parsing = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(reading.as_slice());
    let data: UnitList = csv_parsing
        .records()
        .enumerate()
        .map(|(index, result)| -> io::Result<(UnitKey, UnitContent)> {
            let record = result?;
            let journal: J = record.deserialize(None)?;
            let string = serde_json::to_string(&journal)?;
            let mut unit_key_str = table_name.to_string();
            unit_key_str.push_str(&index.to_string());
            let unit_key = UnitKey::from(unit_key_str.as_str());
            let content = UnitContent::from(string.as_str());
            Ok((unit_key, content))
        })
        .enumerate()
        .map(|(index, datum)| -> (UnitKey, UnitContent) {
            match datum {
                Err(_) => {
                    let mut unit_key_str = table_name.to_string();
                    unit_key_str.push_str(&index.to_string());
                    let unit_key = UnitKey::from(unit_key_str.as_str());
                    let content = UnitContent::String(String::from("json:error"));
                    return (unit_key, content);
                }
                Ok(datum) => datum,
            }
        })
        .take(row_limit)
        .collect();
    return Ok(data);
}

pub fn measure_single_operation<F, A, T>(operate: F, arg: &A) -> Result<f64, Box<dyn Error>>
where
    F: Fn(&A) -> Result<T, Box<dyn Error>>,
{
    let start = Instant::now();
    operate(&arg)?;
    let elapsed = start.elapsed().as_millis();
    Ok(elapsed as f64)
}

pub fn measure_iteration<D, F, T>(
    data: &[D],
    operate: F,
    operation_name: &str,
    report_period: usize,
) -> Result<Vec<(f64, f64)>, Box<dyn Error>>
where
    F: Fn(&D) -> Result<T, Box<dyn Error>>,
{
    let mut start = Instant::now();
    let mut count = 0;
    let total_periods = data.len() / report_period;
    let mut times: Vec<(f64, f64)> = Vec::with_capacity(total_periods + 1);
    for datum in data.iter() {
        operate(datum)?;
        count += 1;
        if count % report_period == 0 {
            let elapsed = start.elapsed().as_millis();
            let average_time = elapsed as f64 / report_period as f64;
            println!(
                "took {}ms to execute {} {} operations ({}/{} done), average {:.2}ms per item",
                elapsed,
                report_period,
                operation_name,
                count,
                data.len(),
                average_time
            );
            start = Instant::now();
            times.push((count as f64, average_time));
        }
    }
    Ok(times)
}

pub fn e2e_verify_correctness(
    grouping: &GroupingLabel,
    table: &[(UnitKey, UnitContent)],
    client: &ImmuxDBHttpClient,
) -> bool {
    for (unit_key, content) in table {
        let (code, actual_output) = client.get_by_key(&grouping, &unit_key).unwrap();
        let expected_output = content;
        let actual_output = UnitContent::from(actual_output.as_str());

        if code != 200 || expected_output != &actual_output {
            return false;
        }
    }

    return true;
}

pub fn get_executor_errors() -> Vec<ExecutorError> {
    let command_errors = get_command_errors();
    let executor_command_errors: Vec<ExecutorError> = command_errors
        .iter()
        .map(|command_error| ExecutorError::CommandError(command_error.clone()))
        .collect();

    let predicate_errors = get_predicate_errors();
    let executor_predicate_errors: Vec<ExecutorError> = predicate_errors
        .iter()
        .map(|predicate_error| ExecutorError::PredicateError(predicate_error.clone()))
        .collect();

    let unit_content_errors = get_unit_content_errors();
    let executor_unit_content_errors: Vec<ExecutorError> = unit_content_errors
        .iter()
        .map(|unit_content_error| ExecutorError::UnitContentError(unit_content_error.clone()))
        .collect();

    let kv_key_errors = get_kvkey_errors();
    let executor_kv_key_errors: Vec<ExecutorError> = kv_key_errors
        .iter()
        .map(|kv_key_error| ExecutorError::KVKeyError(kv_key_error.clone()))
        .collect();

    let kv_errors = get_kv_errors();
    let executor_kv_errors: Vec<ExecutorError> = kv_errors
        .iter()
        .map(|kv_error| ExecutorError::KVError(kv_error.clone()))
        .collect();

    let system_errors = get_system_errors();
    let executor_system_errors: Vec<ExecutorError> = system_errors
        .iter()
        .map(|system_error| ExecutorError::SystemError(system_error.clone()))
        .collect();

    let mut result = vec![
        ExecutorError::ParseIntError(SystemError::ParseIntError),
        ExecutorError::UnexpectedOutcome,
        ExecutorError::ParseExecutorErrorToStringError,
    ];

    result.extend_from_slice(&executor_command_errors);
    result.extend_from_slice(&executor_predicate_errors);
    result.extend_from_slice(&executor_unit_content_errors);
    result.extend_from_slice(&executor_kv_key_errors);
    result.extend_from_slice(&executor_kv_errors);
    result.extend_from_slice(&executor_system_errors);

    result
}

pub fn get_kv_errors() -> Vec<KVError> {
    let system_errors = get_system_errors();
    let kv_system_errors: Vec<KVError> = system_errors
        .iter()
        .map(|system_error| KVError::SystemError(system_error.clone()))
        .collect();

    let log_version_errors = get_log_version_errors();
    let kv_log_version_errors: Vec<KVError> = log_version_errors
        .iter()
        .map(|log_version_error| KVError::LogVersionError(log_version_error.clone()))
        .collect();

    let chain_height_errors = get_chain_height_errors();
    let executor_chain_height_errors: Vec<KVError> = chain_height_errors
        .iter()
        .map(|chain_height_error| KVError::ChainHeightError(chain_height_error.clone()))
        .collect();

    let instruction_errors = get_instruction_errors();
    let executor_instruction_errors: Vec<KVError> = instruction_errors
        .iter()
        .map(|instruction_error| KVError::InstructionError(instruction_error.clone()))
        .collect();

    let mut result = vec![
        KVError::PointToUnexpectedInstruction,
        KVError::ParseKVErrorError,
        KVError::RevertOutOfRange,
        KVError::ParseIntError(SystemError::ParseIntError),
        KVError::IOError(SystemError::IOError),
    ];

    result.extend_from_slice(&kv_system_errors);
    result.extend_from_slice(&kv_log_version_errors);
    result.extend_from_slice(&executor_chain_height_errors);
    result.extend_from_slice(&executor_instruction_errors);

    result
}

pub fn get_chain_height_errors() -> Vec<ChainHeightError> {
    vec![
        ChainHeightError::NegativeChainHeight,
        ChainHeightError::ChainHeightOutOfRange,
        ChainHeightError::ParseChainHeightErrorError,
    ]
}

pub fn get_transaction_manager_errors() -> Vec<TransactionManagerError> {
    vec![
        TransactionManagerError::TransactionNotAlive,
        TransactionManagerError::TransactionIdOutOfRange,
        TransactionManagerError::ParseTransactionManagerErrorError,
    ]
}

pub fn get_error_correction_errors() -> Vec<ErrorCorrectionError> {
    vec![
        ErrorCorrectionError::ParseErrorCorrectionErrorError,
        ErrorCorrectionError::DataWidthNotDivisibleByModulus(3),
    ]
}

pub fn get_command_errors() -> Vec<CommandError> {
    let unit_key_errors = get_unit_key_errors();
    let command_unit_key_errors: Vec<CommandError> = unit_key_errors
        .iter()
        .map(|unit_key_error| CommandError::UnitKeyError(unit_key_error.clone()))
        .collect();

    let varint_errors = get_varint_errors();
    let command_varint_errors: Vec<CommandError> = varint_errors
        .iter()
        .map(|varint_error| CommandError::VarIntError(varint_error.clone()))
        .collect();

    let grouping_errors = get_grouping_label_errors();
    let command_grouping_errors: Vec<CommandError> = grouping_errors
        .iter()
        .map(|grouping_error| CommandError::GroupingErr(grouping_error.clone()))
        .collect();

    let unit_content_errors = get_unit_content_errors();
    let command_unit_content_errors: Vec<CommandError> = unit_content_errors
        .iter()
        .map(|unit_content_error| CommandError::UnitContentErr(unit_content_error.clone()))
        .collect();

    let select_condition_errors = get_select_condition_errors();
    let command_select_condition_errors: Vec<CommandError> = select_condition_errors
        .iter()
        .map(|select_condition_error| {
            CommandError::SelectConditionErr(select_condition_error.clone())
        })
        .collect();

    let mut result = vec![
        CommandError::InvalidPrefix,
        CommandError::ParseCommandErrorToStringError,
    ];

    result.extend_from_slice(&command_unit_key_errors);
    result.extend_from_slice(&command_varint_errors);
    result.extend_from_slice(&command_grouping_errors);
    result.extend_from_slice(&command_unit_content_errors);
    result.extend_from_slice(&command_select_condition_errors);

    result
}

pub fn get_grouping_label_errors() -> Vec<GroupingLabelError> {
    let varint_errors = get_varint_errors();

    let grouping_label_varint_errors: Vec<GroupingLabelError> = varint_errors
        .iter()
        .map(|varint_error| GroupingLabelError::VarIntError(varint_error.clone()))
        .collect();

    let mut result = vec![GroupingLabelError::ParseGroupingLabelErrorError];
    result.extend_from_slice(&grouping_label_varint_errors);

    result
}

pub fn get_predicate_errors() -> Vec<PredicateError> {
    let unit_content_error = get_unit_content_errors();
    let predicate_unit_content_errors: Vec<PredicateError> = unit_content_error
        .iter()
        .map(|content_error| PredicateError::UnitContent(content_error.clone()))
        .collect();

    let varint_errors = get_varint_errors();
    let predicate_malformedbytes_error: Vec<PredicateError> = varint_errors
        .iter()
        .map(|varint_error| PredicateError::MalformedBytes(varint_error.clone()))
        .collect();

    let mut result = vec![
        PredicateError::MalformedTokens,
        PredicateError::UnexpectedToken,
        PredicateError::UnexpectedPrefix(2),
        PredicateError::InsufficientBytes,
        PredicateError::ParsePredicateErrorError,
    ];

    result.extend_from_slice(&predicate_unit_content_errors);
    result.extend_from_slice(&predicate_malformedbytes_error);

    result
}

pub fn get_instruction_errors() -> Vec<InstructionError> {
    let error_correction_errors = get_error_correction_errors();
    let instruction_error_correction_errors: Vec<InstructionError> = error_correction_errors
        .iter()
        .map(|error_correction_error| {
            InstructionError::ErrorCorrection(error_correction_error.clone())
        })
        .collect();

    let varint_errors = get_varint_errors();
    let instruction_varint_errors: Vec<InstructionError> = varint_errors
        .iter()
        .map(|varint_error| InstructionError::VarIntError(varint_error.clone()))
        .collect();

    let mut result = vec![
        InstructionError::PackTooShort(1),
        InstructionError::UnknownPrefix(1),
        InstructionError::MissingPrefixByte,
        InstructionError::UnexpectedECCMode(1),
        InstructionError::KeyExceedsMaxLength,
        InstructionError::UnexpectedMagicNumber([0x01, 0x02, 0x03, 0x04]),
        InstructionError::UnexpectedPackVersion(0x01),
        InstructionError::ParseInstructionErrorError,
    ];

    result.extend_from_slice(&instruction_error_correction_errors);
    result.extend_from_slice(&instruction_varint_errors);

    result
}

pub fn get_unit_content_errors() -> Vec<UnitContentError> {
    vec![
        UnitContentError::EmptyInput,
        UnitContentError::MissingDataBytes,
        UnitContentError::UnexpectedTypePrefix(2),
        UnitContentError::UnexpectedLengthBytes,
        UnitContentError::ParseUnitContentErrorError,
    ]
}

pub fn get_unit_key_errors() -> Vec<UnitKeyError> {
    let varint_errors = get_varint_errors();
    let mut result = vec![UnitKeyError::ParseUnitKeyErrorError];

    for varint_error in varint_errors {
        let unit_key_error = UnitKeyError::VarIntError(varint_error);
        result.push(unit_key_error);
    }

    return result;
}

pub fn get_select_condition_errors() -> Vec<SelectConditionError> {
    let unit_key_errors = get_unit_key_errors();
    let select_condition_unit_key_errors: Vec<SelectConditionError> = unit_key_errors
        .iter()
        .map(|unit_key_error| SelectConditionError::UnitKeyError(unit_key_error.clone()))
        .collect();

    let system_errors = get_system_errors();
    let select_condition_system_errors: Vec<SelectConditionError> = system_errors
        .iter()
        .map(|system_error| SelectConditionError::SystemError(system_error.clone()))
        .collect();

    let predicate_errors = get_predicate_errors();
    let select_condition_predicate_errors: Vec<SelectConditionError> = predicate_errors
        .iter()
        .map(|predicate_error| SelectConditionError::PredicateError(predicate_error.clone()))
        .collect();

    let grouping_errors = get_grouping_label_errors();
    let select_condition_grouping_errors: Vec<SelectConditionError> = grouping_errors
        .iter()
        .map(|grouping_error| SelectConditionError::GroupingError(grouping_error.clone()))
        .collect();

    let mut result = vec![
        SelectConditionError::InvalidPrefix,
        SelectConditionError::ParseSelectConditionErrorError,
        SelectConditionError::FromUtf8Error(SystemError::FromUtf8Error),
    ];

    result.extend_from_slice(&select_condition_unit_key_errors);
    result.extend_from_slice(&select_condition_system_errors);
    result.extend_from_slice(&select_condition_predicate_errors);
    result.extend_from_slice(&select_condition_grouping_errors);

    result
}

pub fn get_system_errors() -> Vec<SystemError> {
    vec![
        SystemError::IOError,
        SystemError::FromUtf8Error,
        SystemError::ParseIntError,
        SystemError::ReceiverError,
        SystemError::ParseSystemErrorError,
    ]
}

pub fn get_varint_errors() -> Vec<VarIntError> {
    vec![
        VarIntError::UnexpectedFormat,
        VarIntError::ParseVarIntErrorError,
    ]
}

pub fn get_kvkey_errors() -> Vec<KVKeyError> {
    let mut result = vec![KVKeyError::ParseKVKeyErrorError];

    let varint_errors = get_varint_errors();

    for varint_error in varint_errors {
        result.push(KVKeyError::VarIntError(varint_error));
    }

    result
}

pub fn get_log_version_errors() -> Vec<LogVersionError> {
    vec![
        LogVersionError::InvalidString,
        LogVersionError::UnexpectedLogVersion,
        LogVersionError::LogVersionParsingError,
        LogVersionError::ParseLogVersionErrorError,
        LogVersionError::ParseIntError(SystemError::ParseIntError),
    ]
}

pub fn get_key_content_pairs() -> UnitList {
    let mut map = HashMap::new();
    map.insert(
        String::from("key1"),
        UnitContent::String(String::from("string in map")),
    );
    map.insert(String::from("key2"), UnitContent::Nil);
    map.insert(String::from("key3"), UnitContent::Bool(false));

    [
        (
            UnitKey::from("key1"),
            UnitContent::String(String::from("this is a string")),
        ),
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

pub fn get_phone_mode_test_predicates() -> Predicate {
    // "I want cheap, small, and Apple phones."
    // this.price <= 500 && this.size <= 6.0 && this.brand == "Apple
    Predicate::Compound(CompoundPredicate::And(vec![
        Predicate::Primitive(PrimitivePredicate::LessThanOrEqual(
            FieldPath::from(vec![String::from("price")]),
            UnitContent::Float64(500.0),
        )),
        Predicate::Primitive(PrimitivePredicate::LessThanOrEqual(
            FieldPath::from(vec![String::from("size")]),
            UnitContent::Float64(6.0),
        )),
        Predicate::Primitive(PrimitivePredicate::Equal(
            FieldPath::from(vec![String::from("brand")]),
            UnitContent::String(String::from("Apple")),
        )),
    ]))
}

// This is paired with get_key_content_pairs
pub fn get_phone_model_fixture() -> (Vec<UnitContent>, Vec<UnitContent>) {
    let expected_satisfied_contents = vec![
        {
            let mut map = HashMap::new();
            map.insert(
                String::from("model"),
                UnitContent::String(String::from("iPhone 23 Mini")),
            );
            map.insert(
                String::from("brand"),
                UnitContent::String(String::from("Apple")),
            );
            map.insert(String::from("size"), UnitContent::Float64(5.5));
            map.insert(String::from("price"), UnitContent::Float64(400.0));
            map.insert(String::from("newest"), UnitContent::Bool(true));

            UnitContent::Map(map)
        },
        {
            let mut map = HashMap::new();
            map.insert(
                String::from("model"),
                UnitContent::String(String::from("iPhone 5")),
            );
            map.insert(
                String::from("brand"),
                UnitContent::String(String::from("Apple")),
            );
            map.insert(String::from("size"), UnitContent::Float64(5.0));
            map.insert(String::from("price"), UnitContent::Float64(100.0));
            map.insert(String::from("newest"), UnitContent::Bool(false));

            UnitContent::Map(map)
        },
    ];

    let unsatisfied_content = vec![
        {
            let mut map = HashMap::new();
            map.insert(
                String::from("model"),
                UnitContent::String(String::from("Shiny new Moto")),
            );
            map.insert(
                String::from("brand"),
                UnitContent::String(String::from("Moto")),
            );
            map.insert(String::from("price"), UnitContent::Float64(100.0));
            map.insert(String::from("newest"), UnitContent::Bool(true));
            map.insert(String::from("size"), UnitContent::Float64(7.0));

            UnitContent::Map(map)
        },
        {
            let mut map = HashMap::new();
            map.insert(
                String::from("model"),
                UnitContent::String(String::from("iPad")),
            );
            map.insert(
                String::from("brand"),
                UnitContent::String(String::from("Apple")),
            );
            map.insert(String::from("size"), UnitContent::Float64(10.0));
            map.insert(String::from("price"), UnitContent::Float64(800.0));
            map.insert(String::from("newest"), UnitContent::Bool(true));

            UnitContent::Map(map)
        },
        {
            let mut map = HashMap::new();
            map.insert(
                String::from("model"),
                UnitContent::String(String::from("Old iPhone MAX Pro")),
            );
            map.insert(
                String::from("brand"),
                UnitContent::String(String::from("Apple")),
            );
            map.insert(String::from("price"), UnitContent::Float64(400.0));
            map.insert(String::from("newest"), UnitContent::Bool(false));
            map.insert(String::from("size"), UnitContent::Float64(7.5));

            UnitContent::Map(map)
        },
    ];
    (expected_satisfied_contents, unsatisfied_content)
}
