use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    terminal::Terminal,
    widgets::{Block, Borders, List, ListItem},
};

use crate::render::help_render::render_help;
use crate::state::files_state::FilesState;

const HELP_TEXT: [&str; 3] = ["Files Help", "TODO", "Add hotkeys relevant to files screen"];

pub fn render_files<B>(
    terminal: &mut Terminal<B>,
    state: &mut FilesState,
    show_help: bool,
) -> Result<(), anyhow::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        let size = f.size();

        let mut items = vec![ListItem::new("../").style(Style::default().fg(Color::LightGreen))];
        for entry in state.files.iter() {
            let text = entry
                .file_name()
                .into_string()
                .expect("Could not parse OsString");

            items.push(ListItem::new(text).style(Style::default().fg(Color::LightGreen)));
        }

        let block = List::new(items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(block, size, &mut state.files_state);

        if show_help {
            render_help(f, &HELP_TEXT);
        }
    })?;

    Ok(())
}
