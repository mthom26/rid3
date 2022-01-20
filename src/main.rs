use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

struct AppState {
    state: ListState,
    items: Vec<String>,
}

impl AppState {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            items: vec![
                "Hello".to_string(),
                "Derp".to_string(),
                "Moar".to_string(),
                "Stuff".to_string(),
                "L337".to_string(),
                "Haxx0r".to_string(),
                "Finished!".to_string(),
            ],
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    let mut app = AppState::new();

    loop {
        // Render
        terminal
            .draw(|f| {
                let size = f.size();

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .split(size);

                let items: Vec<ListItem> = app
                    .items
                    .iter()
                    .map(|item| {
                        ListItem::new(item.clone()).style(Style::default().fg(Color::LightGreen))
                    })
                    .collect();

                let left_block = List::new(items)
                    .block(Block::default().title("Left").borders(Borders::ALL))
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    );

                f.render_stateful_widget(left_block, chunks[0], &mut app.state);

                let chunks_right = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                    .split(chunks[1]);

                let right_block = Block::default().title("Right").borders(Borders::ALL);
                f.render_widget(right_block, chunks_right[0]);

                let input_block = Block::default().title("Input").borders(Borders::ALL);
                f.render_widget(input_block, chunks_right[1]);
            })
            .unwrap();

        // Handle input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => app.prev(),
                    KeyCode::Down => app.next(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    terminal
        .backend_mut()
        .execute(LeaveAlternateScreen)
        .unwrap();
}
