use anyhow::Result;
use aws_config::{profile::profile_file::ProfileFileKind, SdkConfig};
use tracing::{event, Level};

use crate::config::config::AWSConfig;

#[derive(Debug)]
pub struct AwsCallerIdentity {
    pub account: String,
    pub arn: String,
    pub user_id: String,
}

pub async fn create_aws_config(aws_config: &AWSConfig) -> SdkConfig {
    let provider = build_credentials_provider(aws_config);

    let mut loader = aws_config::from_env().credentials_provider(provider);
    if !aws_config.endpoint.is_empty() {
        loader = loader.endpoint_url(aws_config.endpoint.as_str());
    }

    let sdk_config = loader.load().await;
    event!(
        Level::DEBUG,
        "AWS Config for profile {} = {:?}",
        aws_config.endpoint,
        sdk_config
    );

    sdk_config
}

fn build_credentials_provider(
    aws_config: &AWSConfig,
) -> aws_config::profile::ProfileFileCredentialsProvider {
    let profile_files = aws_config::profile::profile_file::ProfileFiles::builder()
        .with_file(
            ProfileFileKind::Credentials,
            aws_config.credentials_path.as_os_str(),
        )
        .build();

    let mut provider =
        aws_config::profile::ProfileFileCredentialsProvider::builder().profile_files(profile_files);

    if !aws_config.profile.is_empty() {
        provider = provider.profile_name(aws_config.profile.as_str());
    }

    provider.build()
}

async fn fetch_caller_identity(config: &SdkConfig) -> Result<AwsCallerIdentity> {
    let client = aws_sdk_sts::Client::new(config);
    let response = client.get_caller_identity().send().await?;

    let identity = AwsCallerIdentity {
        account: response.account().unwrap_or_default().to_string(),
        arn: response.arn().unwrap_or_default().to_string(),
        user_id: response.user_id().unwrap_or_default().to_string(),
    };

    event!(Level::DEBUG, "Caller Identity = {:?}", identity);

    Ok(identity)
}
