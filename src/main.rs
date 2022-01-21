use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{backend::CrosstermBackend, Terminal};

mod render;
mod state;
mod util;
use render::render;
use state::State;

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
    let mut state = State::new(tags);

    loop {
        render(&mut terminal, &mut state)?;

        // Handle input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => state.prev_file(),
                    KeyCode::Down => state.next_file(),
                    KeyCode::Char('w') => state.prev_detail(),
                    KeyCode::Char('s') => state.next_detail(),
                    _ => {}
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
