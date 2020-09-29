use std::env;
use std::path::PathBuf;

use immuxsys::constants as Constants;
use immuxsys::server::errors::ServerResult;
use immuxsys::server::server::run_db_servers;

fn main() -> ServerResult<()> {
    let args: Vec<String> = env::args().collect();
    let args_num = args.len();
    let (log_file_path, http_port, tcp_port) = {
        if args_num == 4 {
            let (path_str, http_port_str, tcp_port_str) = (&args[1], &args[2], &args[3]);
            let path = PathBuf::from(path_str);
            let http_port = http_port_str.parse::<u16>()?;
            let tcp_port = tcp_port_str.parse::<u16>()?;
            (path, http_port, tcp_port)
        } else if args_num == 3 {
            let (path_str, http_port_str) = (&args[1], &args[2]);
            let path = PathBuf::from(path_str);
            let http_port = http_port_str.parse::<u16>()?;
            let tcp_port = Constants::TCP_SERVER_DEFAULT_PORT;
            (path, http_port, tcp_port)
        } else if args_num == 2 {
            let path_str = &args[1];
            let path = PathBuf::from(path_str);
            let http_port = Constants::HTTP_SERVER_DEFAULT_PORT;
            let tcp_port = Constants::TCP_SERVER_DEFAULT_PORT;
            (path, http_port, tcp_port)
        } else {
            (
                PathBuf::from(Constants::DEFAULT_LOG_FILE_PATH),
                Constants::HTTP_SERVER_DEFAULT_PORT,
                Constants::TCP_SERVER_DEFAULT_PORT,
            )
        }
    };

    run_db_servers(&log_file_path, Some(http_port), Some(tcp_port));
    return Ok(());
}
