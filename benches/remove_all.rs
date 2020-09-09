use std::thread;

use immuxsys::constants as Constants;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table_with_size, launch_db_server, measure_single_operation, notified_sleep,
    read_usize_from_arguments,
};

#[derive(Clone)]
struct BenchSpec {
    pub bytes_limit: usize,
    pub port: u16,
}

fn main() {
    let verify_correctness = read_usize_from_arguments(3).unwrap_or(0) > 0;

    let bench_name = "bench_remove_all";
    let bench_specs = [
        BenchSpec {
            bytes_limit: 1024 * 3,
            port: 4440,
        },
        BenchSpec {
            bytes_limit: 1024 * 1024 * 30,
            port: 4441,
        },
        BenchSpec {
            bytes_limit: 1024 * 1024 * 300,
            port: 4442,
        },
    ];

    println!("\nExecuting bench {}", bench_name);

    for (index, bench_spec) in bench_specs.iter().enumerate() {
        let project_name = format!("{}{}", bench_name, index);
        let bench_spec = bench_spec.clone();
        launch_db_server(&project_name, Some(bench_spec.port), None).unwrap();
    }

    notified_sleep(5);

    let mut children_thread = vec![];

    for (index, bench_spec) in bench_specs.iter().enumerate() {
        let bench_spec = bench_spec.clone();
        let project_name = format!("{}{}", bench_name, index);
        let child = thread::spawn(move || {
            let grouping = GroupingLabel::from("census90");
            let (table, read_bytes) = csv_to_json_table_with_size::<CensusEntry>(
                "dev_utils/src/data_models/data-raw/census90.txt",
                &grouping.to_string(),
                b',',
                bench_spec.bytes_limit,
            )
            .unwrap();

            let host = &format!("{}:{}", Constants::SERVER_END_POINT, bench_spec.port);
            let client = ImmuxDBHttpClient::new(host).unwrap();

            for (unit_key, content) in table.iter() {
                client.set_unit(&grouping, unit_key, content).unwrap();
            }

            println!(
                "total {} bytes were inserted to {}",
                read_bytes, project_name
            );

            let total_time =
                measure_single_operation(|_x| client.remove_all().map_err(|err| err.into()), &"")
                    .unwrap();

            println!(
                "took {}ms to execute remove_all {}.",
                total_time, project_name
            );

            if verify_correctness {
                println!("Verifying correctness.");
                for (unit_key, _content) in table.iter() {
                    let (code, actual_output) = client.get_by_key(&grouping, unit_key).unwrap();
                    assert_eq!(code, 200);
                    assert_eq!(actual_output, "Nil");
                }
                println!("Verifying correctness finished");
            }
        });

        children_thread.push(child);
    }

    for child in children_thread {
        assert!(child.join().is_ok());
    }
}
