use std::thread;

use immuxsys::constants as Constants;
use immuxsys::storage::executor::grouping::Grouping;
use immuxsys_client::client::ImmuxDBClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, launch_db, measure_iteration, notified_sleep, read_usize_from_arguments,
};

fn main() {
    let port = 4450;
    let bench_name = "bench_inspect_one";
    let row_limit = read_usize_from_arguments(1).unwrap_or(10_000);
    let report_period = read_usize_from_arguments(2).unwrap_or(1_000);

    println!(
        "\nExecuting bench {}, with tables truncated at row {}, aggregating {} operations",
        bench_name, row_limit, report_period
    );

    thread::spawn(move || launch_db("bench_inspect_one", port));
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
        client.set_unit(&grouping, unit_key, content).unwrap();
    }

    measure_iteration(
        &table,
        |(unit_key, _unit_content)| {
            client
                .inspect_one(&grouping, &unit_key)
                .map_err(|err| err.into())
        },
        "inspect_one",
        report_period,
    )
    .unwrap();
}
