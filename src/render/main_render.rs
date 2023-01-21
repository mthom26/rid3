use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{
    configuration::Config,
    logger::Logger,
    render::{active_list_item, inactive_list_item, list_item, render_logs, render_popup},
    state::main_state::{DetailItem, Focus, MainState},
};

pub fn main_render<B>(
    terminal: &mut Terminal<B>,
    log_state: &Logger,
    app_config: &Config,
    show_logs: bool,
    state: &mut MainState,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();

        let chunks = if show_logs {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(10)].as_ref())
                .split(size)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(0)].as_ref())
                .split(size)
        };

        let chunks_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(chunks[0]);

        // MainState files
        let file_items: Vec<ListItem> = state
            .files
            .iter()
            .map(|item| {
                let text = item.filename.clone();

                ListItem::new(text).style(match item.selected {
                    true => inactive_list_item(app_config),
                    false => list_item(app_config),
                })
            })
            .collect();

        let left_block = List::new(file_items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Files => active_list_item(app_config),
                _ => inactive_list_item(app_config),
            });

        f.render_stateful_widget(left_block, chunks_top[0], &mut state.files_state);

        // MainState details
        let details: Vec<ListItem> = state
            .details
            .iter()
            .map(|item| match item {
                DetailItem::FileName(file_name) => {
                    let text = format!("┳ Filename\n┗ {}\n", file_name);
                    ListItem::new(text).style(list_item(app_config))
                }
                DetailItem::Frame(frame) => {
                    let text = format!("┳ {}\n┗ {}\n", frame.name(), frame.content());
                    ListItem::new(text).style(list_item(app_config))
                }
            })
            .collect();

        let right_block = List::new(details)
            .block(Block::default().title("Details").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Details => active_list_item(app_config),
                _ => inactive_list_item(app_config),
            });
        f.render_stateful_widget(right_block, chunks_top[1], &mut state.details_state);

        // Logs
        if show_logs {
            let log_block = render_logs(app_config, log_state);
            f.render_widget(log_block, chunks[1]);
        }

        // Popup
        if let Some(popup) = state.popup_widget() {
            render_popup(size, f, popup, app_config);
        }
    })?;

    Ok(())
}
