use std::collections::HashMap;

use clap::{builder::PossibleValuesParser, command, Arg};

use crate::ui;

pub(crate) const CREDENTIALS_KEY: &str = "aws.credentialsPath";
pub(crate) const ENDPOINT_KEY: &str = "aws.endpoint";
pub(crate) const LOG_LEVEL_KEY: &str = "logging.level";
pub(crate) const LOG_FILE_PATH: &str = "logging.logFilePath";
pub(crate) const PERFORMANCE_KEY: &str = "performance";

pub fn parse_commands() -> HashMap<String, String> {
    let matches = command!()
        .author("Thomas Berchtold")
        .about("An AWS console within your terminal.")
        .long_about(get_long_about())
        .arg(
            Arg::new(CREDENTIALS_KEY)
                .value_name("CREDENTIALS")
                .short('c')
                .long("credentials")
                .help("Set custom AWS credentials path")
                .long_help(
                    "Path to a custom AWS credentials file. If omitted, default values are used",
                ),
        )
        .arg(
            Arg::new(ENDPOINT_KEY)
                .value_name("ENDPOINT")
                .short('e')
                .long("endpoint")
                .help("Set endpoint for AWS services")
                .long_help(
                    "Alternative endpoint for the invocation of the AWS services",
                ),
        )
        .arg(
            Arg::new(LOG_LEVEL_KEY)
                .value_name("LOG_LEVEL")
                .short('l')
                .long("level")
                .value_parser(PossibleValuesParser::new([
                    "TRACE", "DEBUG", "INFO", "WARN", "ERROR",
                ]))
                .help("Set log level")
                .long_help("Set the Log Level to be used for application logging"),
        )
        .arg(
            Arg::new(LOG_FILE_PATH)
                .value_name("LOG_FILE_PATH")
                .short('f')
                .long("logfile")
                .help("Set log file path")
                .long_help("Set the path where the logfile should be written. If not set, no file logging will take place."),
        )
        .arg(
            Arg::new(PERFORMANCE_KEY)
                .short('p')
                .long("performance")
                .num_args(0)
                .help("Activate performance measurement")
                .long_help("Activate the performance measurement for the rendering and action loop"),
        )
        .get_matches();

    create_arguments(matches)
}

fn create_arguments(matches: clap::ArgMatches) -> HashMap<String, String> {
    let mut arguments = HashMap::new();

    if let Some(aws_credentials_path) = matches.get_one::<String>(CREDENTIALS_KEY) {
        arguments.insert(CREDENTIALS_KEY.into(), aws_credentials_path.clone());
    }

    if let Some(aws_endpoint) = matches.get_one::<String>(ENDPOINT_KEY) {
        arguments.insert(ENDPOINT_KEY.into(), aws_endpoint.clone());
    }

    if let Some(log_level) = matches.get_one::<String>(LOG_LEVEL_KEY) {
        arguments.insert(LOG_LEVEL_KEY.into(), log_level.clone());
    }

    if let Some(log_file_path) = matches.get_one::<String>(LOG_FILE_PATH) {
        arguments.insert(LOG_FILE_PATH.into(), log_file_path.clone());
    }

    if matches.get_flag(PERFORMANCE_KEY) {
        arguments.insert(PERFORMANCE_KEY.into(), "yes".into());
    }

    arguments
}

fn get_long_about() -> String {
    let mut logo = ui::logo::large_logo();
    let text = "\n\naws-console-tui provides a AWS console directly within your terminal to create, browse, or edit your services.";
    logo.push_str(text);
    logo
}
