use log::Level;
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{config::Config, logger::Logger};

pub fn render_logs<'a>(config: &Config, log_state: &Logger) -> List<'a> {
    let log_items: Vec<ListItem> = log_state
        .items
        .lock()
        .unwrap()
        .iter()
        .map(|item| {
            // let s = format!("{} - {}", item.level, item.msg);
            let spans = Spans::from(vec![
                Span::styled(
                    item.level.to_string(),
                    Style::default().fg(match item.level {
                        Level::Error => config.log_error_fg(),
                        Level::Warn => config.log_warn_fg(),
                        Level::Info => config.log_info_fg(),
                        Level::Debug => config.log_trace_fg(), // TODO - Replace this
                        Level::Trace => config.log_trace_fg(),
                    }),
                ),
                Span::raw(" "),
                Span::raw(item.msg.clone()),
            ]);
            ListItem::new(spans)
        })
        .collect();
    let log_block = List::new(log_items).block(Block::default().title("Log").borders(Borders::ALL));

    log_block
}
