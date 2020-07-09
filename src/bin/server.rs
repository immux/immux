use std::path::PathBuf;

use immuxsys::constants as Constants;
use immuxsys::server::errors::ServerResult;
use immuxsys::server::server::run_server;
use immuxsys::storage::executor::executor::Executor;

fn main() -> ServerResult<()> {
    let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
    let executor = Executor::open(&path)?;
    let default_port = 6324;
    run_server(executor, default_port)?;
    return Ok(());
}
