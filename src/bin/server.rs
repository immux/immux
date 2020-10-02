use immuxsys::server::errors::ServerResult;
use immuxsys::server::server::run_db_servers;
use immuxsys::storage::preferences::DBPreferences;

fn read_args() -> DBPreferences {
    let args: Vec<String> = std::env::args().collect();
    return DBPreferences::from_cli_args(&args);
}

fn main() -> ServerResult<()> {
    let prefs = read_args();
    run_db_servers(&prefs);
    return Ok(());
}
