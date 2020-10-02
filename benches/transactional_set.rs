use immuxsys::constants as Constants;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys::storage::transaction_manager::TransactionId;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table, e2e_verify_correctness, launch_test_db_servers, measure_iteration,
    read_usize_from_arguments,
};

fn main() {
    let port = 4402;
    let bench_name = "bench_transactional_set";
    let row_limit = read_usize_from_arguments(1).unwrap_or(50_000);
    let report_period = read_usize_from_arguments(2).unwrap_or(1_000);
    let verify_correctness = read_usize_from_arguments(3).unwrap_or(0) > 0;

    println!(
        "\nExecuting bench {}, with tables truncated at row {}, aggregating {} operations",
        bench_name, row_limit, report_period
    );

    launch_test_db_servers("bench_transactional_set", Some(port), None).unwrap();

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

    measure_iteration(
        &table,
        |(unit_key, unit_content)| {
            let (_, transaction_id_str) = client.create_transaction()?;
            let transaction_id = transaction_id_str.parse::<u64>()?;
            let transaction_id = TransactionId::new(transaction_id);
            client.transactional_set_unit(&grouping, unit_key, unit_content, &transaction_id)?;
            client
                .commit_transaction(&transaction_id)
                .map_err(|err| err.into())
        },
        "transaction_set",
        report_period,
    )
    .unwrap();

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
