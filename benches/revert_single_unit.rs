use std::collections::HashMap;

use immuxsys::constants as Constants;
use immuxsys::storage::chain_height::ChainHeight;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_client::ImmuxDBClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, launch_test_db_servers, measure_iteration, read_usize_from_arguments,
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

    launch_test_db_servers("bench_revert_single_unit", Some(port), None).unwrap();

    let grouping = GroupingLabel::from("census90");
    let table = csv_to_json_table::<CensusEntry>(
        "dev_utils/src/data_models/data-raw/census90.txt",
        format!("{}", &grouping).as_str(),
        b',',
        row_limit,
    )
    .unwrap();

    let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
    let client = ImmuxDBHttpClient::new(host).unwrap();

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
                let expected_output = format!("{}", expected_table.get(&unit_key).unwrap());
                assert_eq!(code, 200);
                assert_eq!(
                    UnitContent::from(expected_output.as_str()),
                    UnitContent::from(actual_output.as_str())
                );
            } else {
                let (code, _actual_output) = client.get_by_key(&grouping, &unit_key).unwrap();
                assert_eq!(code, 500);
            }
        }
        println!("Verifying correctness finished");
    }
}
