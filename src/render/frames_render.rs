use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
    widgets::{Block, Borders, List, ListItem},
};
use tui_logger::TuiWidgetState;

use crate::{
    config::Config,
    render::{help_render::render_help, inactive_list_item, list_item, logs_render::render_logs},
    state::{frame_data::SUPPORTED_FRAMES, frames_state::FramesState},
};

const HELP_TEXT: [&str; 2] = ["`q` - Quit", "`a` - Add selected frame"];

pub fn render_frames<B>(
    terminal: &mut Terminal<B>,
    state: &mut FramesState,
    show_help: bool,
    logger_state: &TuiWidgetState,
    config: &Config,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();
        /*
        ┌──────────────────────┬───────────────────┐
        │                      │                   │
        │                      │                   │
        │     chunks_top[0]    │   chunks_top[1]   │
        │                      │                   │
        │                      │                   │
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

        // Frames list
        let frames: Vec<ListItem> = SUPPORTED_FRAMES
            .iter()
            .map(|frame| ListItem::new(frame.name).style(list_item(config)))
            .collect();

        let frames_block = List::new(frames)
            .block(Block::default().title("Frames").borders(Borders::ALL))
            .highlight_style(inactive_list_item(config));

        f.render_stateful_widget(frames_block, chunks_top[0], &mut state.frames_state);

        // TODO - Add description and info for the highlighted frame
        let description_block = Block::default()
            .title("Frame Description")
            .borders(Borders::ALL);

        f.render_widget(description_block, chunks_top[1]);

        // Log block
        let mut log_widget = render_logs(config);
        log_widget.state(logger_state);
        f.render_widget(log_widget, chunks[1]);

        if show_help {
            render_help(f, &HELP_TEXT, config);
        }
    })?;

    Ok(())
}
