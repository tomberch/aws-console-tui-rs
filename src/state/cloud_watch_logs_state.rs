#[derive(Clone, Debug, Default)]
pub struct CloudWatchState {
    pub log_groups: Vec<CloudWatchLogGroup>,
    pub selected_log_group: Option<CloudWatchLogGroup>,
}

#[derive(Clone, Debug, Default)]
pub struct CloudWatchLogGroup {
    pub arn: String,
    pub name: Option<String>,
    pub date_created: Option<i64>,
    pub retention_days: Option<i32>,
    pub stored_bytes: Option<i64>,
    pub log_streams: Vec<CloudWatchLogStream>,
}

#[derive(Clone, Debug, Default)]
pub struct CloudWatchLogStream {
    pub arn: String,
    pub log_stream_name: Option<String>,
    pub creation_time: Option<i64>,
    pub first_event_timestamp: Option<i64>,
    pub last_event_timestamp: Option<i64>,
    pub last_ingestion_time: Option<i64>,
    pub stored_bytes: Option<i64>,
}
