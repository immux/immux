use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::time::Duration;
use std::{io, thread};

use immuxsys::server::server::run_server;
use immuxsys::storage::executor::executor::Executor;

pub fn reset_db_dir(path: &str) -> io::Result<()> {
    println!("Initializing database in {}", path);
    create_dir_all(&path)?;
    remove_dir_all(&path)?;
    println!("Existing test data removed");
    return Ok(());
}

pub fn launch_db(project_name: &str, port: u16) -> io::Result<()> {
    let data_root = format!("/tmp/{}/", project_name);
    reset_db_dir(&data_root)?;

    let path = PathBuf::from(data_root);
    match Executor::open(&path) {
        Ok(executor) => match run_server(executor, port) {
            Ok(_) => println!("Database started"),
            Err(error) => {
                println!("Cannot start database: {:?}", error);
            }
        },
        Err(error) => println!("Cannot start database: {:?}", error),
    }
    Ok(())
}

pub fn notified_sleep(sec: u16) -> () {
    println!("Waiting {}s...", sec);
    thread::sleep(Duration::from_secs(sec as u64));
}
