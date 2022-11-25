use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
    text::Span,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::{
    config::Config,
    logger::Logger,
    render::{
        active_list_item, help_render::render_help, inactive_list_item, list_item,
        logs_render::render_logs, popup_render::get_block,
    },
    state::main_state::{Focus, MainState},
};

const HELP_TEXT: [&str; 7] = [
    "`q` - Quit",
    "`c` - Remove all files",
    "`d` - Remove selected files/frame",
    "`s` - Select highlighted file",
    "`a` - Select/Deselect all files",
    "`u` - Update file names",
    "`w` - Write changes",
];

const FRAME_HELP_TEXT: [&str; 2] = ["`Esc` - Back", "`w` - Save frame"];

pub fn render_main<B>(
    terminal: &mut Terminal<B>,
    state: &mut MainState,
    show_help: bool,
    config: &Config,
    log_state: &Logger,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();
        /*
        ┌──────────────────────┬───────────────────┐
        │                      │                   │
        │                      │  chunks_right[0]  │
        │     chunks_top[0]    │                   │
        │                      ├───────────────────┤
        │                      │  chunks_right[1]  │
        ├──────────────────────┴───────────────────┤
        │                                          │
        │                chunks[1]                 │
        │                                          │
        └──────────────────────────────────────────┘
        */
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(10)].as_ref())
            .split(size);

        let chunks_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(chunks[0]);

        // Active files list
        let items: Vec<ListItem> = state
            .files
            .iter()
            .map(|item| {
                let text = match item.path.to_str() {
                    Some(t) => t
                        .split("/")
                        .last()
                        .unwrap_or("!Problem unwrapping filename!"),
                    None => "!Unknown Artist!",
                };
                ListItem::new(text).style(match item.selected {
                    true => inactive_list_item(config),
                    false => list_item(config),
                })
            })
            .collect();

        let left_block = List::new(items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Files => active_list_item(config),
                _ => inactive_list_item(config),
            });

        f.render_stateful_widget(left_block, chunks_top[0], &mut state.files_state);

        // Details list
        let filename = format!("┳ Filename\n┗ {}\n", state.details_filename);
        let mut right_items = vec![ListItem::new(filename).style(list_item(config))];

        for item in state.details.iter().map(|item| {
            let text = format!("┳ {}\n┗ {}\n", item.name(), item.content());
            ListItem::new(text).style(list_item(config))
        }) {
            right_items.push(item);
        }

        let right_block = List::new(right_items)
            .block(Block::default().title("Details").borders(Borders::ALL))
            .highlight_style(match state.focus {
                Focus::Details => active_list_item(config),
                _ => inactive_list_item(config),
            });
        f.render_stateful_widget(right_block, chunks_top[1], &mut state.details_state);

        // Log block
        let log_block = render_logs(config, log_state);
        f.render_widget(log_block, chunks[1]);

        // Render popup
        match state.focus {
            Focus::Edit => {
                let items: Vec<ListItem> = state
                    .popup
                    .items
                    .iter()
                    .map(|item| {
                        let text = format!("┳ {}\n┗ {}\n", item.0, item.1);
                        ListItem::new(text).style(list_item(config))
                    })
                    .collect();
                let (block, input_block, rect, input_rect) = get_block(f, config);
                let list = List::new(items)
                    .block(block)
                    .highlight_style(active_list_item(config));

                f.render_widget(Clear, rect);
                f.render_widget(Clear, input_rect);
                f.render_widget(input_block, input_rect);
                f.render_stateful_widget(list, rect, &mut state.popup.state);
            }
            Focus::EditInput => {
                let items: Vec<ListItem> = state
                    .popup
                    .items
                    .iter()
                    .map(|item| {
                        let text = format!("┳ {}\n┗ {}\n", item.0, item.1);
                        ListItem::new(text).style(list_item(config))
                    })
                    .collect();
                let (block, input_block, rect, input_rect) = get_block(f, config);
                let list = List::new(items)
                    .block(block)
                    .highlight_style(active_list_item(config));

                let input_block = Paragraph::new(Span::raw(&state.popup.input)).block(input_block);
                f.render_widget(Clear, rect);
                f.render_widget(Clear, input_rect);
                f.render_widget(input_block, input_rect);
                f.render_stateful_widget(list, rect, &mut state.popup.state);
                f.set_cursor(
                    input_rect.x + state.popup.cursor_pos as u16 + 1,
                    input_rect.y + 1,
                );
            }
            _ => {}
        }

        if show_help {
            render_help(
                f,
                match state.focus {
                    Focus::Edit => &FRAME_HELP_TEXT,
                    _ => &HELP_TEXT,
                },
                config,
            );
        }
    })?;

    Ok(())
}
