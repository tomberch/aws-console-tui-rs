use const_format::formatcp;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::{Color, Style};

pub struct TuiConfig<'a> {
    pub tick_rate_in_ms: u64,
    pub root: Style,
    pub focus_border: Style,
    pub non_focus_border: Style,
    pub key_config: KeyConfig<'a>,
    pub list_config: ListConfig<'a>,
    pub services: Services<'a>,
    pub messages: Messages<'a>,
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
}

pub const TUI_CONFIG: TuiConfig = TuiConfig {
    tick_rate_in_ms: 250,
    root: Style::new().bg(DARK_BLUE),
    focus_border: Style::new().fg(Color::Green),
    non_focus_border: Style::new().fg(Color::White),
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
    },
};

const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
// const LIGHT_BLUE: Color = Color::Rgb(64, 96, 192);
// const LIGHT_YELLOW: Color = Color::Rgb(192, 192, 96);
// const LIGHT_GREEN: Color = Color::Rgb(64, 192, 96);
// const LIGHT_RED: Color = Color::Rgb(192, 96, 96);
// const RED: Color = Color::Indexed(160);
// const BLACK: Color = Color::Indexed(232); // not really black, often #080808
// const DARK_GRAY: Color = Color::Indexed(238);
// const MID_GRAY: Color = Color::Indexed(244);
// const LIGHT_GRAY: Color = Color::Indexed(250);
// const WHITE: Color = Color::Indexed(255); // not really white, often #eeeeee
