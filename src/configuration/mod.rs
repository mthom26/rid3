use std::path::PathBuf;

use config::{self, File, FileFormat};
use directories::ProjectDirs;
use log::{error, info, warn};
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
        info!("{}", "Building new config");
        let conf = if let Some(s) = get_config_file_string() {
            config::Config::builder()
                .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
                .add_source(File::with_name(&s))
                .build()
                .unwrap()
        } else {
            config::Config::builder()
                .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
                .build()
                .unwrap()
        };

        match conf.clone().try_deserialize::<Config>() {
            Ok(c) => c,
            Err(e) => {
                error!("Using default config - {}", e);
                config::Config::builder()
                    .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
                    .build()
                    .unwrap()
                    .try_deserialize::<Config>()
                    .unwrap()
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

pub fn get_config_file_string() -> Option<String> {
    if let Some(dirs) = ProjectDirs::from("rid3", "rid3", "rid3") {
        let path_buf = dirs.config_dir().join("config.toml");

        if path_buf.exists() {
            Some(path_buf.into_os_string().into_string().unwrap())
        } else {
            warn!("No user config.toml file found. Using default config.");
            None
        }
    } else {
        warn!("No user config directory found. Using default config.");
        None
    }
}

pub fn get_config_dir() -> Option<PathBuf> {
    if let Some(dirs) = ProjectDirs::from("rid3", "rid3", "rid3") {
        Some(PathBuf::from(dirs.config_dir()))
    } else {
        None
    }
}
