use std::path::PathBuf;
use std::thread;

use immuxsys::constants as Constants;
use immuxsys::storage::executor::executor::Executor;
use immuxsys::storage::executor::grouping_label::GroupingLabel;
use immuxsys_client::http_client::ImmuxDBHttpClient;
use immuxsys_dev_utils::data_models::census90::CensusEntry;
use immuxsys_dev_utils::dev_utils::{
    csv_to_json_table_with_size, launch_db_server, measure_single_operation, notified_sleep,
};

#[derive(Clone)]
struct BenchSpec {
    pub bytes_limit: usize,
    pub port: u16,
}

fn main() {
    let bench_name = "bench_launch_db";
    let bench_specs = [
        BenchSpec {
            bytes_limit: 1024 * 3,
            port: 20028,
        },
        BenchSpec {
            bytes_limit: 1024 * 1024 * 3,
            port: 20029,
        },
        BenchSpec {
            bytes_limit: 1024 * 1024 * 300,
            port: 20030,
        },
    ];

    println!("\nExecuting bench {}", bench_name);

    for (index, bench_spec) in bench_specs.iter().enumerate() {
        let project_name = format!("{}_{}", bench_name, index);
        let bench_spec = bench_spec.clone();
        thread::spawn(move || {
            launch_db_server(&project_name, bench_spec.port).unwrap();
        });
    }

    notified_sleep(5);

    let mut children_thread = vec![];

    for (index, bench_spec) in bench_specs.iter().enumerate() {
        let bench_spec = bench_spec.clone();
        let project_name = format!("{}_{}", bench_name, index);
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

            let data_root = format!("/tmp/{}/", project_name);
            let path = PathBuf::from(data_root);
            let total_time = measure_single_operation(
                |path| Executor::open(path).map_err(|err| err.into()),
                &path,
            )
            .unwrap();

            println!(
                "took {}ms to execute Executor::open {}.",
                total_time, project_name
            );
        });

        children_thread.push(child);
    }

    for child in children_thread {
        assert!(child.join().is_ok());
    }
}
