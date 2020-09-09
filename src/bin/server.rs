use std::path::PathBuf;
use std::sync::Arc;

use immuxsys::constants as Constants;
use immuxsys::server::errors::{ServerError, ServerResult};
use immuxsys::server::server::run_server;

fn main() -> ServerResult<()> {
    let default_http_port = Constants::HTTP_SERVER_DEFAULT_PORT;
    let default_tcp_port = Constants::TCP_SERVER_DEFAULT_PORT;
    let path = Arc::new(PathBuf::from(Constants::TEMP_LOG_FILE_PATH));
    let handlers = run_server(path, Some(default_http_port), Some(default_tcp_port));

    for handler in handlers {
        match handler.join() {
            Ok(server_result) => return server_result,
            Err(_error) => return Err(ServerError::ThreadError),
        }
    }
    return Ok(());
}
