use std::error::Error;
use std::thread;

use immuxsys::constants as Constants;
use immuxsys::storage::executor::grouping::Grouping;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys::storage::executor::unit_key::UnitKey;
use immuxsys_client::client::ImmuxDBClient;
use immuxsys_dev_utils::data_models::business::Business;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, e2e_verify_correctness, launch_db, measure_iteration, notified_sleep,
    read_usize_from_arguments,
};

fn main() {
    let port = 20020;
    let bench_name = "anzsic06";
    let row_limit = read_usize_from_arguments(1).unwrap_or(100_000);
    let report_period = read_usize_from_arguments(2).unwrap_or(10_000);
    let verify_correctness = read_usize_from_arguments(3).unwrap_or(0) > 0;

    println!(
        "\nExecuting bench {}, with tables truncated at row {}, aggregating {} operations",
        bench_name, row_limit, report_period
    );

    thread::spawn(move || launch_db("bench_anzsic06", port));
    notified_sleep(5);

    let paths = vec!["anzsic06"];
    let dataset: Vec<(Grouping, Vec<(UnitKey, UnitContent)>)> = paths
        .iter()
        .map(
            |table_name| -> (
                Grouping,
                Result<Vec<(UnitKey, UnitContent)>, Box<dyn Error>>,
            ) {
                let csv_path = format!("dev_utils/src/data_models/data-raw/{}.csv", table_name);

                let data = match table_name.as_ref() {
                    "anzsic06" => {
                        csv_to_json_table::<Business>(&csv_path, table_name, b',', row_limit)
                    }
                    _ => panic!("Unexpected table {}", table_name),
                };
                return (Grouping::new(table_name.as_bytes()), data);
            },
        )
        .map(|result| -> (Grouping, Vec<(UnitKey, UnitContent)>) {
            match result.1 {
                Err(error) => {
                    eprintln!("CSV error: {}", error);
                    return (Grouping::new("error".as_bytes()), vec![]);
                }
                Ok(table) => return (result.0, table),
            }
        })
        .collect();

    let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
    let client = ImmuxDBClient::new(host).unwrap();

    for (table_name, table) in dataset.iter() {
        println!(
            "Loading table '{}', total records {}",
            &table_name.to_string(),
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

    for (table_name, table) in dataset.iter() {
        println!(
            "Reading table '{}', total records {}",
            &table_name.to_string(),
            table.len()
        );
        measure_iteration(
            table,
            |(unit_key, _unit_content)| {
                let res = client.get_by_key(table_name, unit_key).unwrap();
                return Ok(res.1);
            },
            "get_by_key",
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
