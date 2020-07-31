use std::collections::HashMap;
use std::thread;

use immuxsys::constants as Constants;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys_client::client::ImmuxDBClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, launch_db, measure_iteration, notified_sleep, read_usize_from_arguments,
};

fn main() {
    let port = 4401;
    let bench_name = "bench_revert_single_unit";
    let row_limit = read_usize_from_arguments(1).unwrap_or(10_000);
    let report_period = read_usize_from_arguments(2).unwrap_or(1_000);
    let verify_correctness = read_usize_from_arguments(3).unwrap_or(0) > 0;
    let target_height = ChainHeight::new(100);

    println!(
        "\nExecuting bench {}, with tables truncated at row {}, aggregating {} operations",
        bench_name, row_limit, report_period
    );

    thread::spawn(move || launch_db("bench_revert_single_unit", port));
    notified_sleep(5);

    let grouping = GroupingLabel::from("census90");
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

    measure_iteration(
        &table,
        |(unit_key, _unit_content)| {
            client
                .revert_one(&grouping, unit_key, &target_height)
                .map_err(|err| err.into())
        },
        "revert_one",
        report_period,
    )
    .unwrap();

    if verify_correctness {
        println!("Verifying correctness.");
        let mut expected_table = HashMap::new();

        for (unit_key, content) in &table[0..target_height.as_u64() as usize + 1] {
            expected_table.insert(unit_key, content);
        }

        for (unit_key, _content) in table.iter() {
            if expected_table.contains_key(unit_key) {
                let (code, actual_output) = client.get_by_key(&grouping, &unit_key).unwrap();
                let expected_output = expected_table.get(&unit_key).unwrap().to_string();
                assert_eq!(code, 200);
                assert_eq!(expected_output, actual_output);
            } else {
                let (code, _actual_output) = client.get_by_key(&grouping, &unit_key).unwrap();
                assert_eq!(code, 500);
            }
        }
        println!("Verifying correctness finished");
    }
}
