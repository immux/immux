use immuxsys::constants as Constants;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::executor::unit_content::UnitContent;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, e2e_verify_correctness, launch_test_db_servers, measure_iteration,
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

    launch_test_db_servers("bench_remove_single_unit", Some(port), None).unwrap();

    let grouping = GroupingLabel::from("census90");
    let table = csv_to_json_table::<CensusEntry>(
        "dev_utils/src/data_models/data-raw/census90.txt",
        &grouping.to_string(),
        b',',
        row_limit,
    )
    .unwrap();

    let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
    let client = ImmuxDBHttpClient::new(host).unwrap();

    for (unit_key, content) in table.iter() {
        client.set_unit(&grouping, &unit_key, &content).unwrap();
    }

    println!(
        "Removing single unit in table '{}', total records {}",
        &grouping,
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
                return (unit_key.to_owned(), UnitContent::Nil);
            })
            .collect();
        assert!(e2e_verify_correctness(&grouping, &expected_table, &client));
        println!("Verifying correctness finished");
    }
}
