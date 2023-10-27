use tokio::sync::mpsc::UnboundedSender;

use crate::{
    repository::{ec2::EC2Repository, login::LoginRepository},
    state::appstate::{AWSService, AppState, Profile, ProfileSource},
    ui::config::TUI_CONFIG,
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
            ProfileAction::SelectProfile { profile: profile } => {
                app_state.status_state.action_pending = true;
                app_state.status_state.message = TUI_CONFIG.messages.pending_action.into();
                app_state.status_state.err_message = "".into();
                let _ = state_tx.send(app_state.clone());
                let _ = ProfileActionHandler::handle_select_profile(&profile, app_state).await;
                app_state.status_state.action_pending = false;
            }
        }
    }
    async fn handle_select_profile(
        (profile_name, profile_source): &(String, ProfileSource),
        app_state: &mut AppState,
    ) {
        if let Some(active_profile) = app_state.active_profile.take() {
            if active_profile.err_message.is_empty() {
                app_state
                    .profile_state
                    .profiles
                    .insert(active_profile.name.clone(), active_profile);
            }
        }

        let profile = match app_state.profile_state.profiles.remove(profile_name) {
            Some(profile) => profile,
            None => {
                let config = LoginRepository::create_aws_config(
                    profile_name,
                    profile_source,
                    &app_state.aws_config,
                )
                .await;
                let result = LoginRepository::fetch_caller_identity(&config).await;
                match result {
                    Ok(identity) => {
                        let mut profile = Profile {
                            name: profile_name.into(),
                            source: profile_source.to_owned(),
                            selected_region: config.region().map(|region| region.as_ref().into()),
                            sdk_config: config,
                            account: identity.account,
                            user: identity.user_id,
                            err_message: "".into(),
                            err_message_backtrace: "".into(),
                            regions: vec![],
                            selected_service: AWSService::None,
                        };
                        match EC2Repository::describe_regions(
                            &app_state.aws_config,
                            &profile.sdk_config,
                        )
                        .await
                        {
                            Ok(regions) => profile.regions = regions.clone(),
                            Err(err) => {
                                profile = Profile {
                                    name: profile_name.into(),
                                    source: profile_source.to_owned(),
                                    sdk_config: profile.sdk_config,
                                    account: "".into(),
                                    user: "".into(),
                                    err_message: TUI_CONFIG.messages.pending_action.into(),
                                    err_message_backtrace: format!("{:?}", err),
                                    regions: vec![],
                                    selected_region: None,
                                    selected_service: AWSService::None,
                                }
                            }
                        };

                        profile
                    }
                    Err(err) => Profile {
                        name: profile_name.into(),
                        source: profile_source.to_owned(),
                        sdk_config: config,
                        account: "".into(),
                        user: "".into(),
                        err_message: format!("Error {}. Press <CRL-m> for more information.", err),
                        err_message_backtrace: format!("{:?}", err),
                        regions: vec![],
                        selected_region: None,
                        selected_service: AWSService::None,
                    },
                }
            }
        };

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
    }
}
