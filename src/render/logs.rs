use log::Level;
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{configuration::Config, logger::Logger};

pub fn render_logs<'a>(config: &Config, log_state: &Logger) -> List<'a> {
    let index = *log_state.index.lock().expect("Could not acquire lock");
    let mut last_index = log_state
        .items
        .lock()
        .expect("Could not acquire lock")
        .len();
    if last_index > 0 {
        last_index -= 1;
    }

    let log_items: Vec<ListItem> = if last_index == 0 {
        vec![]
    } else {
        log_state.items.lock().unwrap()[index..=last_index]
            .iter()
            .map(|item| {
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
            .collect()
    };

    let title = format!("Logs {}/{}", index + 1, last_index + 1);
    List::new(log_items).block(Block::default().title(title).borders(Borders::ALL))
}
