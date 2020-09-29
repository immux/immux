use std::collections::HashMap;
use std::error::Error;
use std::fs::read;
use std::fs::{create_dir_all, remove_dir_all};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;
use std::{io, thread};

use immuxsys::server::errors::ServerResult;
use immuxsys::server::server::run_db_servers;
use immuxsys::storage::executor::filter::{
    Filter, FilterOperands, FilterOperator, FilterUnit, LogicalOperator,
};
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_client::ImmuxDBClient;

pub use serde::de::{Deserialize, DeserializeOwned};
pub use serde::ser::Serialize;

pub type UnitList = Vec<(UnitKey, UnitContent)>;

pub fn reset_db_dir(path: &str) -> io::Result<()> {
    println!("Initializing database in {}", path);
    create_dir_all(&path)?;
    remove_dir_all(&path)?;
    println!("Existing test data removed");
    return Ok(());
}

pub fn launch_db_server(
    project_name: &str,
    http_port: Option<u16>,
    tcp_port: Option<u16>,
) -> ServerResult<Vec<ServerResult<()>>> {
    let data_root = format!("/tmp/{}/", project_name);
    reset_db_dir(&data_root)?;

    let path = PathBuf::from(data_root);
    let server_results = run_db_servers(&path, http_port, tcp_port);

    return Ok(server_results);
}

pub fn notified_sleep(sec: u16) -> () {
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

pub fn get_filter() -> Filter {
    let mut filter_units = Vec::new();
    let mut logical_operators = Vec::new();

    let filter_units_vec = vec![
        FilterUnit {
            operator: FilterOperator::GreaterOrEqual,
            operands: FilterOperands {
                map_key: String::from("price"),
                unit_content: UnitContent::Float64(1200.0),
            },
        },
        FilterUnit {
            operator: FilterOperator::LessOrEqual,
            operands: FilterOperands {
                map_key: String::from("size"),
                unit_content: UnitContent::Float64(13.0),
            },
        },
        FilterUnit {
            operator: FilterOperator::Equal,
            operands: FilterOperands {
                map_key: String::from("used"),
                unit_content: UnitContent::Bool(true),
            },
        },
        FilterUnit {
            operator: FilterOperator::Equal,
            operands: FilterOperands {
                map_key: String::from("name"),
                unit_content: UnitContent::String(String::from("Apple")),
            },
        },
    ];

    let logical_operators_vec = vec![
        LogicalOperator::And,
        LogicalOperator::Or,
        LogicalOperator::Or,
    ];

    filter_units.extend(filter_units_vec);
    logical_operators.extend(logical_operators_vec);

    let filter = Filter {
        filter_units,
        logical_operators,
    };

    return filter;
}
