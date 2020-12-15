use std::path::PathBuf;

use immuxsys::constants as Constants;
use immuxsys::server::errors::ServerResult;
use immuxsys::server::server::run_db_servers;

fn main() -> ServerResult<()> {
    let default_http_port = Constants::HTTP_SERVER_DEFAULT_PORT;
    let default_tcp_port = Constants::TCP_SERVER_DEFAULT_PORT;
    let path = PathBuf::from(Constants::TEMP_LOG_FILE_PATH);
    run_db_servers(&path, Some(default_http_port), Some(default_tcp_port));
    return Ok(());
}
