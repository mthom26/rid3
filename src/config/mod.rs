use std::fs;

use home::home_dir;
use log::{error, warn};
use serde::Deserialize;
use tui::style::Color;

mod theme;
use theme::Theme;

const DEFAULT_CONFIG: &str = r#"
    [theme]
    list_item_fg = 'LightGreen'
    list_item_bg = 'Reset'
    active_list_item_fg = 'LightYellow'
    active_list_item_bg = 'DarkGray'
    inactive_list_item_fg = 'LightGreen'
    inactive_list_item_bg = 'DarkGray'

    help_border = 'LightYellow'

    log_error_fg = 'Red'
    log_info_fg = 'Blue'
    log_trace_fg = 'DarkGray'
    log_warn_fg = 'Yellow'
"#;

#[derive(Debug, Deserialize)]
pub struct Config {
    theme: Theme,
}

impl Config {
    pub fn new() -> Self {
        let config_str = match home_dir() {
            Some(mut path) => {
                path.push(".config");
                path.push("rid3");
                path.push("config.toml");

                match fs::read_to_string(path) {
                    Ok(s) => Some(s),
                    Err(_) => None,
                }
            }
            None => None,
        };

        match config_str {
            Some(s) => match toml::from_str(&s) {
                Ok(config) => config,
                Err(e) => {
                    error!("Failed to parse config file - {}", e);
                    warn!("Using default config.");
                    toml::from_str(DEFAULT_CONFIG).unwrap()
                }
            },
            None => {
                warn!("No config file found. Using default config.");
                toml::from_str(DEFAULT_CONFIG).unwrap()
            }
        }
    }

    pub fn list_item_fg(&self) -> Color {
        self.theme.list_item_fg.into()
    }

    pub fn list_item_bg(&self) -> Color {
        self.theme.list_item_bg.into()
    }

    pub fn active_list_item_fg(&self) -> Color {
        self.theme.active_list_item_fg.into()
    }

    pub fn active_list_item_bg(&self) -> Color {
        self.theme.active_list_item_bg.into()
    }

    pub fn inactive_list_item_fg(&self) -> Color {
        self.theme.inactive_list_item_fg.into()
    }

    pub fn inactive_list_item_bg(&self) -> Color {
        self.theme.inactive_list_item_bg.into()
    }

    pub fn help_border(&self) -> Color {
        self.theme.help_border.into()
    }

    pub fn log_error_fg(&self) -> Color {
        self.theme.log_error_fg.into()
    }

    pub fn log_info_fg(&self) -> Color {
        self.theme.log_info_fg.into()
    }

    pub fn log_trace_fg(&self) -> Color {
        self.theme.log_trace_fg.into()
    }

    pub fn log_warn_fg(&self) -> Color {
        self.theme.log_warn_fg.into()
    }
}
