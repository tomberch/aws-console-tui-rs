use anyhow::anyhow;
use aws_config::SdkConfig;
use aws_sdk_cloudwatchlogs::{config, Client};
use tracing::{event, Level};

use crate::{config::config::AWSConfig, state::cloud_watch_logs_state::CloudWatchLogGroup};

pub struct CloudWatchLogsRepository;

impl CloudWatchLogsRepository {
    pub async fn describe_log_groups(
        aws_config: &AWSConfig,
        sdk_config: &SdkConfig,
        next_token: Option<String>,
    ) -> anyhow::Result<Vec<CloudWatchLogGroup>> {
        let client = CloudWatchLogsRepository::get_client(aws_config, sdk_config);
        let mut log_group_client = client.describe_log_groups();
        if let Some(next_token_string) = next_token {
            log_group_client = log_group_client.next_token(next_token_string);
        };

        match log_group_client.send().await {
            Ok(response) => {
                let log_groups = response
                    .log_groups()
                    .unwrap_or_default()
                    .iter()
                    .map(|group| CloudWatchLogGroup {
                        arn: group.arn().unwrap().into(),
                        name: group.log_group_name().map(|name| name.into()),
                        date_created: group.creation_time(),
                        retention_days: group.retention_in_days(),
                        stored_bytes: group.stored_bytes(),
                        log_streams: vec![],
                    })
                    .collect();

                event!(Level::DEBUG, "{:?}", log_groups);

                Ok(log_groups)
            }
            Err(err) => {
                event!(Level::WARN, "Error CloudWatch Logs Repository {:?}", err);
                Err(anyhow!(err))
            }
        }
    }

    fn get_client(aws_config: &AWSConfig, sdk_config: &SdkConfig) -> Client {
        let mut client_builder = config::Builder::from(sdk_config);

        if !aws_config.endpoint.is_empty() {
            client_builder = client_builder.endpoint_url(&aws_config.endpoint);
        }

        Client::from_conf(client_builder.build())
    }
}
