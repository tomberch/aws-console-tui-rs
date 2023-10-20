use tokio::sync::mpsc::UnboundedSender;

use crate::{
    repository::{ec2::EC2Repository, login::LoginRepository},
    state::appstate::{AppState, Profile},
};

use super::actions::ProfileAction;

pub struct ProfileActionHandler;

impl ProfileActionHandler {
    pub async fn handle(
        state_tx: UnboundedSender<AppState>,
        action: ProfileAction,
        app_state: &mut AppState,
    ) {
        match action {
            ProfileAction::SelectProfile {
                profile_name: profile,
            } => {
                app_state.status_state.message = "Connecting to profile".into();
                app_state.status_state.err_message = "".into();
                let _ = state_tx.send(app_state.clone());
                let _ = ProfileActionHandler::handle_select_profile(&profile, app_state).await;
            }
        }
    }
    async fn handle_select_profile(
        profile_name: &str,
        app_state: &mut AppState,
    ) -> anyhow::Result<()> {
        if let Some(active_profile) = app_state.active_profile.take() {
            if active_profile.err_message.is_empty() {
                app_state
                    .profile_state
                    .profiles
                    .insert(active_profile.name.clone(), active_profile);
            }
        }

        let mut profile = match app_state.profile_state.profiles.remove(profile_name) {
            Some(profile) => profile,
            None => {
                let config =
                    LoginRepository::create_aws_config(profile_name, &app_state.aws_config).await;
                match LoginRepository::fetch_caller_identity(&config).await {
                    Ok(identity) => Profile {
                        name: profile_name.into(),
                        sdk_config: config,
                        account: identity.account,
                        user: identity.user_id,
                        err_message: "".into(),
                        err_message_backtrace: "".into(),
                        regions: vec![],
                    },
                    Err(err) => Profile {
                        name: profile_name.into(),
                        sdk_config: config,
                        account: "".into(),
                        user: "".into(),
                        err_message: format!("Error {}. Press <CRL-m> for more information.", err),
                        err_message_backtrace: format!("{:?}", err),
                        regions: vec![],
                    },
                }
            }
        };

        profile.regions =
            EC2Repository::describe_regions(&app_state.aws_config, &profile.sdk_config).await?;

        if profile.err_message.is_empty() {
            app_state.status_state.message = format!(
                "Profile: {}, Account: {}, User: {}",
                profile_name,
                profile.account.clone(),
                profile.user.clone()
            );
            app_state.status_state.err_message = String::default();
            app_state.status_state.err_message_backtrace = String::default();
            let _ = app_state.active_profile.insert(profile);
        } else {
            app_state.status_state.message = String::default();
            app_state.status_state.err_message = profile.err_message.clone();
            app_state.status_state.err_message_backtrace = profile.err_message_backtrace.clone();
        }

        Ok(())
    }
}
