use const_format::formatcp;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::{Color, Style};

pub struct TuiConfig<'a> {
    pub tick_rate_in_ms: u64,
    pub sys_info_update_rate_in_sec: u64,
    pub performance_measure_rate_in_sec: u64,
    pub key_config: KeyConfig<'a>,
    pub list_config: ListConfig<'a>,
    pub services: Services<'a>,
    pub breadcrumbs: Breadcrumbs<'a>,
    pub menu: Menu<'a>,
    pub messages: Messages<'a>,
    pub theme: Theme<'a>,
}

pub struct KeyConfig<'a> {
    pub cycle_forward: KeyDescription<'a>,
    pub cycle_backward: KeyDescription<'a>,
    pub focus_profiles: KeyDescription<'a>,
    pub focus_regions: KeyDescription<'a>,
    pub focus_services: KeyDescription<'a>,
    pub focus_aws_service: KeyDescription<'a>,
}

pub struct KeyDescription<'a> {
    pub key_string: &'a str,
    pub key_code: KeyCode,
    pub key_modifier: KeyModifiers,
}

const CTRL: &str = "CTRL";
const TAB: &str = "TAB";
const SHIFT: &str = "SHIFT";

pub struct ListConfig<'a> {
    pub selected_symbol: &'a str,
    pub selected_style: Style,
    pub selection_up: KeyCode,
    pub selection_down: KeyCode,
    pub do_selection: KeyCode,
}

pub struct Services<'a> {
    pub cloud_watch_logs: &'a str,
    pub dynamodb: &'a str,
    pub eks: &'a str,
    pub s3_simple_storage_service: &'a str,
    pub service_catalog: &'a str,
}

pub struct Messages<'a> {
    pub pending_action: &'a str,
    pub error_caller_identity: &'a str,
    pub error_describe_cloud_watch_log_groups: &'a str,
}

pub struct Breadcrumbs<'a> {
    pub profiles: &'a str,
    pub regions: &'a str,
    pub services: &'a str,
    pub cloud_watch_logs: &'a str,
}

pub struct Menu<'a> {
    pub quit: MenuItemText<'a>,
    pub tab: MenuItemText<'a>,
    pub back_tab: MenuItemText<'a>,
    pub details: MenuItemText<'a>,
    pub up: MenuItemText<'a>,
    pub down: MenuItemText<'a>,
    pub select: MenuItemText<'a>,
}

pub struct MenuItemText<'a> {
    pub title: &'a str,
    pub command: &'a str,
    pub color_index: usize,
}

pub struct Theme<'a> {
    pub background: Color,
    pub border: Color,
    pub border_highlight: Color,
    pub command_colors: &'a [Color],
    pub toolbar_info_topic: Color,
    pub status_message_text: Color,
    pub error_message_text: Color,
    pub breadcrumb_foreground: Color,
    pub breadcrumb_background: Color,
    pub breadcrumb_background_active: Color,
}

pub const TUI_CONFIG: TuiConfig = TuiConfig {
    tick_rate_in_ms: 250,
    sys_info_update_rate_in_sec: 5,
    performance_measure_rate_in_sec: 5,
    key_config: KeyConfig {
        cycle_forward: KeyDescription {
            key_string: TAB,
            key_code: KeyCode::Tab,
            key_modifier: KeyModifiers::NONE,
        },
        cycle_backward: KeyDescription {
            key_string: formatcp!("{}-{}", SHIFT, TAB),
            key_code: KeyCode::BackTab,
            key_modifier: KeyModifiers::NONE,
        },
        focus_profiles: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, "1"),
            key_code: KeyCode::Char('1'),
            key_modifier: KeyModifiers::CONTROL,
        },
        focus_regions: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, "2"),
            key_code: KeyCode::Char('2'),
            key_modifier: KeyModifiers::CONTROL,
        },
        focus_services: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, "3"),
            key_code: KeyCode::Char('3'),
            key_modifier: KeyModifiers::CONTROL,
        },
        focus_aws_service: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, "4"),
            key_code: KeyCode::Char('4'),
            key_modifier: KeyModifiers::CONTROL,
        },
    },
    list_config: ListConfig {
        selected_style: Style::new().fg(Color::LightGreen),
        selected_symbol: ">",
        selection_up: KeyCode::Up,
        selection_down: KeyCode::Down,
        do_selection: KeyCode::Enter,
    },
    services: Services {
        cloud_watch_logs: "CloudWatch Logs",
        dynamodb: "DynamoDB",
        eks: "EKS Elastic Kubernetes Service",
        s3_simple_storage_service: "S3 Simple Storage Service",
        service_catalog: "Service Catalog",
    },
    messages: Messages {
        pending_action: "Pending action. Please wait ...",
        error_caller_identity:
            "Error: Cloud not fetch caller identity. Press <CTRL-m> for more information",
        error_describe_cloud_watch_log_groups:
            "Error: CloudWatch Log Groups could not be fetched. Press <CTRL-m> for more information",
    },
    breadcrumbs: Breadcrumbs {
        profiles: " <profiles> ",
        regions: " <regions> ",
        services: " <services> ",
        cloud_watch_logs: "<logs>",
    },
    menu: Menu {
        up: MenuItemText {
            title: "prev element",
            command: "<up>",
            color_index: 1,
        },
        down: MenuItemText {
            title: "next element",
            command: "<down",
            color_index: 1,
        },
        select: MenuItemText {
            title: "select",
            command: "<enter>",
            color_index: 1,
        },
        quit: MenuItemText {
            title: "quit",
            command: "<ctrl-c>",
            color_index: 1,
        },
        tab: MenuItemText {
            title: "cycle forward",
            command: "<tab>",
            color_index: 1,
        },
        back_tab: MenuItemText {
            title: "cycle backward",
            command: "<shift-tab>",
            color_index: 1,
        },
        details: MenuItemText {
            title: "Details",
            command: "<d>",
            color_index: 0,
        },
    },
    theme: Theme {
        background: Color::Indexed(232),
        border: Color::Rgb(0, 126, 200),
        border_highlight: Color::Rgb(135, 206, 250),
        command_colors: &[Color::Rgb(255, 27, 255), Color::Rgb(30, 144, 255)],
        toolbar_info_topic: Color::Rgb(231, 120, 0),
        status_message_text: Color::Rgb(226, 199, 160),
        error_message_text: Color::Rgb(204, 0, 0),
        breadcrumb_foreground: Color::Indexed(232),
        breadcrumb_background: Color::Rgb(0, 255, 255),
        breadcrumb_background_active: Color::Rgb(255, 165, 0),
    },
};
