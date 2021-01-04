use std::error::Error;

use immuxsys::constants as Constants;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_client::ImmuxDBClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, e2e_verify_correctness, launch_test_db_servers, measure_iteration,
    read_usize_from_arguments,
};
use immuxsys_dev_utils::least_squares::solve;

fn main() {
    let port = 20021;
    let bench_name = "census90";
    let row_limit = read_usize_from_arguments(1).unwrap_or(50_000);
    let report_period = read_usize_from_arguments(2).unwrap_or(10_000);
    let verify_correctness = read_usize_from_arguments(3).unwrap_or(0) > 0;

    println!(
        "\nExecuting bench {}, with tables truncated at row {}, aggregating {} operations",
        bench_name, row_limit, report_period
    );

    launch_test_db_servers("bench_census90", Some(port), None).unwrap();

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
    let insert = || -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
        measure_iteration(
            &table,
            |(unit_key, unit_content)| {
                client
                    .set_unit(&grouping, &unit_key, &unit_content)
                    .map_err(|err| err.into())
            },
            "set_unit",
            report_period,
        )
    };

    let get = || -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
        measure_iteration(
            &table,
            |(unit_key, _unit_content)| {
                client
                    .get_by_key(&grouping, &unit_key)
                    .map(|(_, _)| {})
                    .map_err(|err| err.into())
            },
            "get_by_key",
            report_period,
        )
    };

    let time_insert_1 = insert().unwrap();
    let (k_insert_1, _) = solve(&time_insert_1);
    println!("Gained {:.2e} ms per item on average", k_insert_1);

    let time_get_1 = get().unwrap();
    let (k_get_1, _) = solve(&time_get_1);
    println!("Gained {:.2e} ms per item on average", k_get_1);

    if verify_correctness {
        println!("Verifying correctness");
        assert!(e2e_verify_correctness(
            &grouping,
            &table.as_slice(),
            &client
        ));
        println!("Verifying correctness finished");
    }
}
