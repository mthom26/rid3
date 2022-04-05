use tui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tui_logger::TuiLoggerWidget;

pub fn render_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_debug(Style::default().fg(Color::LightBlue))
        .style_error(Style::default().fg(Color::Red))
        .style_info(Style::default().fg(Color::Blue))
        .style_trace(Style::default().fg(Color::DarkGray))
        .style_warn(Style::default().fg(Color::Yellow))
        .output_timestamp(Some("%I:%M:%S%P".to_string()))
        .output_separator(' ')
        .output_file(false)
        .output_target(false)
        .output_line(false)
        .output_level(None)
        .block(Block::default().title("Logs").borders(Borders::ALL))
}
