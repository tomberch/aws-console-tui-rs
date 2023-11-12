use std::{sync::Arc, time::Duration};

use sysinfo::{CpuExt, CpuRefreshKind, System, SystemExt};
use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        RwLock,
    },
    time::Instant,
};
use tokio_util::sync::CancellationToken;

use crate::{
    config::config::AppConfig,
    state::actions::{
        cloud_watch_logs_action_handler::CloudWatchLogsActionHandler,
        profile_action_handler::ProfileActionHandler, region_action_handler::RegionActionHandler,
        service_action_handler::ServiceActionHandler,
    },
    ui::config::TUI_CONFIG,
};

use super::{actions::actions::Action, appstate::AppState};

pub struct StateManager {
    app_config: AppConfig,
    state_tx: UnboundedSender<Arc<RwLock<AppState>>>,
}

impl StateManager {
    pub fn new(app_config: AppConfig) -> (Self, UnboundedReceiver<Arc<RwLock<AppState>>>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<Arc<RwLock<AppState>>>();
        (
            StateManager {
                app_config,
                state_tx,
            },
            state_rx,
        )
    }

    pub async fn run(
        self,
        mut action_rx: UnboundedReceiver<Action>,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let app_state = Arc::new(RwLock::new(AppState::new(&self.app_config)));
        let mut sys_info_interval =
            tokio::time::interval(Duration::from_secs(TUI_CONFIG.sys_info_update_rate_in_sec));

        let mut sys_info = System::new_all();
        sys_info.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
        sys_info.refresh_memory();

        // set the initial state once
        self.state_tx.send(app_state.clone())?;

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                break
                    }

                _ = sys_info_interval.tick() => {
                    let mut mut_app_state = app_state.write().await;
                    sys_info.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
                    sys_info.refresh_memory();
                    mut_app_state.toolbar_state.memory_usage = format!("{:.2} %", sys_info.used_memory() as f64 / sys_info.total_memory() as f64 * 100.0);
                    mut_app_state.toolbar_state.cpu_usage = format!("{:.2} %", sys_info.global_cpu_info().cpu_usage());

                }

                Some(action) = action_rx.recv() => {
                    let start = Instant::now();
                    let mut mut_app_state = app_state.write().await;
                    match action {
                        Action::SetFocus { component_type } => {
                            mut_app_state.focus_component = component_type;

                        },
                        Action::SetBreadcrumbs {breadcrumbs } => {mut_app_state.status_state.breadcrumbs = breadcrumbs },
                        Action::SetMenu { menu_items } => {mut_app_state.toolbar_state.menu_items = menu_items },
                        Action::RenderDuration{ duration } => {mut_app_state.measure_state.render_duration = format!("{:?}", duration) },
                        Action::Profile{ action } => {ProfileActionHandler::handle(action, &mut mut_app_state).await },
                        Action::Region{action} => {RegionActionHandler::handle(action, &mut mut_app_state) },
                        Action::Service{ action }=>{ServiceActionHandler::handle( action, &mut mut_app_state).await },
                        Action::CloudWatchLogs {action} =>{CloudWatchLogsActionHandler::handle(action, &mut mut_app_state).await },
                    }
                    mut_app_state.measure_state.action_duration = format!("{:?}", start.elapsed());
                }
            }

            self.state_tx.send(app_state.clone())?;
        }
        Ok(())
    }
}
