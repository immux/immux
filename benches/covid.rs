use std::error::Error;

use immuxsys::constants as Constants;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_client::ImmuxDBClient;
use immuxsys_dev_utils::data_models::covid::Covid;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, e2e_verify_correctness, launch_test_db_servers, measure_iteration,
    read_usize_from_arguments, UnitList,
};

fn main() {
    let port = 4396;
    let bench_name = "covid";
    let row_limit = read_usize_from_arguments(1).unwrap_or(50_000);
    let report_period = read_usize_from_arguments(2).unwrap_or(1_000);
    let verify_correctness = read_usize_from_arguments(3).unwrap_or(0) > 0;

    println!(
        "\nExecuting bench {}, with tables truncated at row {}, aggregating {} operations",
        bench_name, row_limit, report_period
    );

    launch_test_db_servers("bench_covid", Some(port), None).unwrap();

    let paths = vec!["covid"];

    let dataset: Vec<(GroupingLabel, UnitList)> = paths
        .iter()
        .map(
            |table_name| -> (GroupingLabel, Result<UnitList, Box<dyn Error>>) {
                let csv_path = format!("dev_utils/src/data_models/data-raw/{}.csv", table_name);
                let data = match table_name.as_ref() {
                    "covid" => csv_to_json_table::<Covid>(&csv_path, table_name, b',', row_limit),
                    _ => panic!("Unexpected table {}", table_name),
                };
                return (GroupingLabel::from(*table_name), data);
            },
        )
        .map(|result| -> (GroupingLabel, UnitList) {
            match result.1 {
                Err(error) => {
                    eprintln!("CSV error: {}", error);
                    return (GroupingLabel::from("error"), vec![]);
                }
                Ok(table) => return (result.0, table),
            }
        })
        .collect();

    let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
    let client = ImmuxDBHttpClient::new(host).unwrap();

    for (table_name, table) in dataset.iter() {
        println!(
            "Loading table '{}', total records {}",
            &table_name,
            table.len()
        );
        measure_iteration(
            table,
            |(unit_key, unit_content)| {
                client
                    .set_unit(table_name, unit_key, unit_content)
                    .map_err(|err| err.into())
            },
            "set_unit",
            report_period,
        )
        .unwrap();
    }

    if verify_correctness {
        println!("Verifying correctness.");
        for (table_name, table) in &dataset {
            assert!(e2e_verify_correctness(table_name, table, &client));
        }
        println!("Verifying correctness finished");
    }
}
