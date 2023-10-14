use const_format::formatcp;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::{Color, Style};

pub struct TuiConfig<'a> {
    pub root: Style,
    pub focus_border: Style,
    pub non_focus_border: Style,
    pub key_config: KeyConfig<'a>,
    pub list_config: ListConfig<'a>,
}

pub struct KeyConfig<'a> {
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

const CTRL: &str = &"CTRL";
const FOCUS_PROFILES_CHAR: char = 'l';
const FOCUS_REGIONS_CHAR: char = 'r';
const FOCUS_SERVICES_CHAR: char = 's';
const FOCUS_AWS_SERVICES_CHAR: char = 'a';

pub struct ListConfig<'a> {
    pub selected_symbol: &'a str,
    pub selected_style: Style,
}

pub const TUI_CONFIG: TuiConfig = TuiConfig {
    root: Style::new().bg(DARK_BLUE),
    focus_border: Style::new().fg(Color::Green),
    non_focus_border: Style::new().fg(Color::White),
    key_config: KeyConfig {
        focus_profiles: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, FOCUS_PROFILES_CHAR),
            key_code: KeyCode::Char(FOCUS_PROFILES_CHAR),
            key_modifier: KeyModifiers::CONTROL,
        },
        focus_regions: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, FOCUS_REGIONS_CHAR),
            key_code: KeyCode::Char(FOCUS_REGIONS_CHAR),
            key_modifier: KeyModifiers::CONTROL,
        },
        focus_services: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, FOCUS_SERVICES_CHAR),
            key_code: KeyCode::Char(FOCUS_SERVICES_CHAR),
            key_modifier: KeyModifiers::CONTROL,
        },
        focus_aws_service: KeyDescription {
            key_string: formatcp!("{}-{}", CTRL, FOCUS_AWS_SERVICES_CHAR),
            key_code: KeyCode::Char(FOCUS_AWS_SERVICES_CHAR),
            key_modifier: KeyModifiers::CONTROL,
        },
    },
    list_config: ListConfig {
        selected_style: Style::new().fg(Color::LightGreen),
        selected_symbol: &">",
    },
};

const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
const LIGHT_BLUE: Color = Color::Rgb(64, 96, 192);
const LIGHT_YELLOW: Color = Color::Rgb(192, 192, 96);
const LIGHT_GREEN: Color = Color::Rgb(64, 192, 96);
const LIGHT_RED: Color = Color::Rgb(192, 96, 96);
const RED: Color = Color::Indexed(160);
const BLACK: Color = Color::Indexed(232); // not really black, often #080808
const DARK_GRAY: Color = Color::Indexed(238);
const MID_GRAY: Color = Color::Indexed(244);
const LIGHT_GRAY: Color = Color::Indexed(250);
const WHITE: Color = Color::Indexed(255); // not really white, often #eeeeee
