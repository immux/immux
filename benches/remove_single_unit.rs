use std::thread;

use immuxsys::constants as Constants;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys_client::client::ImmuxDBClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys::storage::executor::grouping::Grouping;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, e2e_verify_correctness, launch_db, measure_iteration, notified_sleep,
    read_usize_from_arguments, UnitList,
};

fn main() {
    let port = 4400;
    let bench_name = "bench_remove_single_unit";
    let row_limit = read_usize_from_arguments(1).unwrap_or(50_000);
    let report_period = read_usize_from_arguments(2).unwrap_or(1_000);
    let verify_correctness = read_usize_from_arguments(3).unwrap_or(0) > 0;

    println!(
        "\nExecuting bench {}, with tables truncated at row {}, aggregating {} operations",
        bench_name, row_limit, report_period
    );

    thread::spawn(move || launch_db("bench_remove_single_unit", port));
    notified_sleep(5);

    let grouping = Grouping::new("census90".as_bytes());
    let table = csv_to_json_table::<CensusEntry>(
        "dev_utils/src/data_models/data-raw/census90.txt",
        &grouping.to_string(),
        b',',
        row_limit,
    )
    .unwrap();

    let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
    let client = ImmuxDBClient::new(host).unwrap();

    for (unit_key, content) in table.iter() {
        client.set_unit(&grouping, &unit_key, &content).unwrap();
    }

    println!(
        "Removing single unit in table '{}', total records {}",
        &grouping.to_string(),
        table.len()
    );
    measure_iteration(
        &table,
        |(unit_key, _unit_content)| {
            client
                .remove_one(&grouping, unit_key)
                .map_err(|err| err.into())
        },
        "remove_one",
        report_period,
    )
    .unwrap();

    if verify_correctness {
        println!("Verifying correctness.");
        let expected_table: UnitList = table
            .iter()
            .map(|(unit_key, _content)| {
                return (unit_key.to_owned(), UnitContent::String("".to_string()));
            })
            .collect();
        assert!(e2e_verify_correctness(&grouping, &expected_table, &client));
        println!("Verifying correctness finished");
    }
}
