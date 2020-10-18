use std::convert::TryFrom;
use std::path::PathBuf;

use crate::constants as Constants;
use crate::storage::ecc::ECCMode;
use crate::storage::log_version::LogVersion;

const CLI_ARG_DATA_DIR: &str = "--data-dir=";
const CLI_ARG_ECC_MODE: &str = "--ecc-mode=";
const CLI_ARG_HTTP_PORT: &str = "--http-port=";
const CLI_ARG_TCP_PORT: &str = "--tcp-port=";

#[derive(Clone, Debug)]
pub struct DBPreferences {
    pub log_dir: PathBuf,
    pub ecc_mode: ECCMode,
    pub http_port: Option<u16>,
    pub tcp_port: Option<u16>,
    pub db_version: LogVersion,
}

impl DBPreferences {
    // Same as default(), except dir is changed
    pub fn default_at_dir(log_dir: &str) -> Self {
        let mut prefs = Self::default();
        prefs.log_dir = PathBuf::from(log_dir);
        return prefs;
    }

    pub fn from_cli_args(cli_args: &[String]) -> Self {
        let mut prefs = DBPreferences::default();
        for arg in cli_args {
            if arg.starts_with(CLI_ARG_DATA_DIR) {
                prefs.log_dir = PathBuf::from(arg.replace(CLI_ARG_DATA_DIR, ""));
            } else if arg.starts_with(CLI_ARG_ECC_MODE) {
                let desired_mode = arg.replace(CLI_ARG_ECC_MODE, "");
                match desired_mode.as_str() {
                    "TMR" => prefs.ecc_mode = ECCMode::TMR,
                    _ => (),
                }
            } else if arg.starts_with(CLI_ARG_HTTP_PORT) {
                match arg.replace(CLI_ARG_HTTP_PORT, "").parse() {
                    Ok(port) => prefs.http_port = Some(port),
                    _ => (),
                }
            } else if arg.starts_with(CLI_ARG_TCP_PORT) {
                match arg.replace(CLI_ARG_TCP_PORT, "").parse() {
                    Ok(port) => prefs.tcp_port = Some(port),
                    _ => (),
                }
            }
        }
        return prefs;
    }
}

impl Default for DBPreferences {
    fn default() -> Self {
        let version_str = env!("CARGO_PKG_VERSION");
        let db_version = match LogVersion::try_from(version_str) {
            Ok(version) => version,
            Err(_err) => LogVersion::new(0, 0, 0),
        };

        return Self {
            log_dir: PathBuf::from(Constants::MAIN_LOG_DEFAULT_DIR_UNIX),
            ecc_mode: ECCMode::Identity,
            http_port: Some(Constants::HTTP_SERVER_DEFAULT_PORT),
            tcp_port: Some(Constants::TCP_SERVER_DEFAULT_PORT),
            db_version,
        };
    }
}

#[cfg(test)]
mod preference_parsing_tests {
    use std::path::PathBuf;

    use crate::storage::ecc::ECCMode;
    use crate::storage::preferences::DBPreferences;

    #[test]
    fn parse_empty_arg() {
        let parsed_preference = DBPreferences::from_cli_args(&[]);
        let default_preference = DBPreferences::default();
        assert_eq!(parsed_preference.log_dir, default_preference.log_dir);
        assert_eq!(parsed_preference.tcp_port, default_preference.tcp_port);
        assert_eq!(parsed_preference.http_port, default_preference.http_port);
        assert_eq!(parsed_preference.ecc_mode, default_preference.ecc_mode);
    }

    #[test]
    fn parse_full_specification() {
        let args_raw = "--data-dir=/tmp/immux --tcp-port=8888 --http-port=2939 --ecc-mode=TMR";
        let args: Vec<String> = args_raw.split(" ").map(|s| s.into()).collect();
        let parsed_preference = DBPreferences::from_cli_args(&args);
        assert_eq!(parsed_preference.log_dir, PathBuf::from("/tmp/immux"));
        assert_eq!(parsed_preference.tcp_port, Some(8888));
        assert_eq!(parsed_preference.http_port, Some(2939));
        assert_eq!(parsed_preference.ecc_mode, ECCMode::TMR);
    }
}