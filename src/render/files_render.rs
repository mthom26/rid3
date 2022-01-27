use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    terminal::Terminal,
    widgets::{Block, Borders, List, ListItem},
};

use crate::state::files_state::FilesState;

pub fn render_files<B>(
    terminal: &mut Terminal<B>,
    state: &mut FilesState,
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
    })?;

    Ok(())
}
