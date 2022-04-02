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
        .block(Block::default().title("Logs").borders(Borders::ALL))
}
