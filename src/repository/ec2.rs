use aws_config::SdkConfig;
use aws_sdk_ec2::{config, Client};

use crate::config::config::AWSConfig;

pub struct EC2Repository;

impl EC2Repository {
    pub async fn describe_regions(
        aws_config: &AWSConfig,
        config: &SdkConfig,
    ) -> anyhow::Result<Vec<String>> {
        let client = &EC2Repository::create_client(aws_config, config);
        let regions_option = client.describe_regions().send().await?;

        let result: Vec<String> = regions_option
            .regions()
            .iter()
            .map(|rg| rg.region_name().unwrap().to_string())
            .collect::<Vec<String>>();

        Ok(result)
    }

    fn create_client(aws_config: &AWSConfig, config: &SdkConfig) -> aws_sdk_ec2::Client {
        let mut client_builder = config::Builder::from(config);

        if !aws_config.endpoint.is_empty() {
            client_builder = client_builder.endpoint_url(&aws_config.endpoint);
        }

        Client::from_conf(client_builder.build())
    }
}
