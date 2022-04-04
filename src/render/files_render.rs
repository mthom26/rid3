use tui::{
    backend::Backend,
    style::{Color, Style},
    terminal::Terminal,
    widgets::{Block, Borders, List, ListItem},
};

use crate::render::{help_render::render_help, inactive_list_item};
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

            let style = if entry.file_type().unwrap().is_dir() {
                Style::default().fg(Color::LightBlue)
            } else {
                Style::default().fg(Color::LightGreen)
            };

            items.push(ListItem::new(text).style(style));
        }

        let block = List::new(items)
            .block(Block::default().title("Files").borders(Borders::ALL))
            .highlight_style(inactive_list_item());

        f.render_stateful_widget(block, size, &mut state.files_state);

        if show_help {
            render_help(f, &HELP_TEXT);
        }
    })?;

    Ok(())
}
