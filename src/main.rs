use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{backend::CrosstermBackend, Terminal};

mod render;
mod state;
mod util;
use render::{files_render::render_files, main_render::render_main};
use state::{files_state::FilesState, main_state::MainState, AppEvent, ScreenState};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    let tags = util::get_id3s().await?;

    let mut screen_state = ScreenState::Main;
    let mut show_help = false;
    let mut main_state = MainState::new(tags);
    let mut files_state = FilesState::new()?;

    loop {
        // Render
        match screen_state {
            ScreenState::Main => render_main(&mut terminal, &mut main_state, show_help)?,
            ScreenState::Files => render_files(&mut terminal, &mut files_state, show_help)?,
        }

        // Handle input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read()? {
                match screen_state {
                    ScreenState::Main => match main_state.handle_input(&key) {
                        AppEvent::Quit => break,
                        AppEvent::NewScreenState(s) => screen_state = s,
                        AppEvent::NewHelpState => show_help = !show_help,
                        _ => {}
                    },
                    ScreenState::Files => match files_state.handle_input(&key) {
                        AppEvent::Quit => break,
                        AppEvent::NewScreenState(s) => screen_state = s,
                        AppEvent::AddFiles(mut tags) => main_state.add_files(&mut tags),
                        AppEvent::NewHelpState => show_help = !show_help,
                        _ => {}
                    },
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    Ok(())
}
