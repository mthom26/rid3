use tui::{
    style::Style,
    widgets::{Block, Borders},
};
use tui_logger::TuiLoggerWidget;

use crate::config::Config;

pub fn render_logs<'a>(config: &Config) -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        // .style_debug(Style::default().fg(Color::LightBlue))
        .style_error(Style::default().fg(config.log_error_fg()))
        .style_info(Style::default().fg(config.log_info_fg()))
        .style_trace(Style::default().fg(config.log_trace_fg()))
        .style_warn(Style::default().fg(config.log_warn_fg()))
        .output_timestamp(Some("%I:%M:%S%P".to_string()))
        .output_separator(' ')
        .output_file(false)
        .output_target(false)
        .output_line(false)
        .output_level(None)
        .block(Block::default().title("Logs").borders(Borders::ALL))
}
