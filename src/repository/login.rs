use anyhow::anyhow;
use aws_config::{profile::profile_file::ProfileFileKind, SdkConfig};

use tracing::{event, Level};

use crate::{config::config::AWSConfig, state::appstate::ProfileSource};

#[derive(Debug)]
pub struct AwsCallerIdentity {
    pub account: String,
    pub arn: String,
    pub user_id: String,
}

pub struct LoginRepository;

impl LoginRepository {
    pub async fn create_aws_config(
        profile_name: &str,
        profile_source: &ProfileSource,
        aws_config: &AWSConfig,
    ) -> SdkConfig {
        let sdk_config = match profile_source {
            ProfileSource::Environment => LoginRepository::build_env_config(aws_config).await,
            ProfileSource::CredentialsFile => {
                LoginRepository::build_credentials_config(profile_name, aws_config).await
            }
            ProfileSource::ConfigFile => {
                LoginRepository::build_sso_config(profile_name, aws_config).await
            }
        };

        event!(
            Level::DEBUG,
            "AWS Config for profile {} = {:?}",
            aws_config.endpoint,
            sdk_config
        );

        sdk_config
    }

    pub async fn fetch_caller_identity(config: &SdkConfig) -> anyhow::Result<AwsCallerIdentity> {
        let client = aws_sdk_sts::Client::new(config);
        match client.get_caller_identity().send().await {
            Ok(response) => {
                let identity = AwsCallerIdentity {
                    account: response.account().unwrap_or_default().to_string(),
                    arn: response.arn().unwrap_or_default().to_string(),
                    user_id: response.user_id().unwrap_or_default().to_string(),
                };
                event!(Level::DEBUG, "Caller Identity = {:?}", identity);
                Ok(identity)
            }
            Err(err) => {
                event!(Level::WARN, "Error Caller Identity = {:?}", err);
                Err(anyhow!(err))
            }
        }
    }

    async fn build_env_config(aws_config: &AWSConfig) -> SdkConfig {
        let mut loader = aws_config::from_env();
        if !aws_config.endpoint.is_empty() {
            loader = loader.endpoint_url(aws_config.endpoint.as_str());
        }

        loader.load().await
    }

    async fn build_credentials_config(profile_name: &str, aws_config: &AWSConfig) -> SdkConfig {
        let mut path = aws_config.file_path.clone();
        path.push("credentials");

        let profile_files = aws_config::profile::profile_file::ProfileFiles::builder()
            .with_file(ProfileFileKind::Credentials, path.as_os_str())
            .build();

        let provider = aws_config::profile::ProfileFileCredentialsProvider::builder()
            .profile_files(profile_files)
            .profile_name(profile_name);

        let mut loader = aws_config::from_env().credentials_provider(provider.build());
        if !aws_config.endpoint.is_empty() {
            loader = loader.endpoint_url(aws_config.endpoint.as_str());
        }

        loader.load().await
    }

    async fn build_sso_config(profile_name: &str, aws_config: &AWSConfig) -> SdkConfig {
        let mut loader = aws_config::from_env().profile_name(profile_name);
        if !aws_config.endpoint.is_empty() {
            loader = loader.endpoint_url(aws_config.endpoint.as_str());
        }

        loader.load().await
    }
}
