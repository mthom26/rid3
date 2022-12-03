use std::{env, io, path::PathBuf, sync::Mutex, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::{debug, LevelFilter};
use tokio::{
    sync::{mpsc, watch},
    time::sleep,
};
use tui::{backend::CrosstermBackend, Terminal};

mod args;
mod config;
mod logger;
mod render;
mod state;
mod util;
use args::get_args;
use config::Config;
use logger::Logger;
use render::{files_render::render_files, frames_render::render_frames, main_render::render_main};
use state::{
    files_state::FilesState, frames_state::FramesState, main_state::MainState, AppEvent,
    ScreenState,
};

static LOGGER: Logger = Logger {
    items: Mutex::new(Vec::new()),
    index: Mutex::new(0),
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = get_args();
    let dir = match args.get_one::<String>("path") {
        Some(p) => PathBuf::from(p),
        None => env::current_dir()?,
    };

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);

    let config = Config::new();

    let mut screen_state = ScreenState::Main;
    let mut show_help = false;
    let mut main_state = MainState::new();
    let mut files_state = FilesState::new(dir)?;
    let mut frames_state = FramesState::new();

    let (input_tx, mut input_rx) = mpsc::channel(32);
    let (timer_tx, mut timer_rx) = mpsc::channel(32);
    let (quit_tx, quit_rx) = watch::channel(());
    let quit_rx1 = quit_rx.clone();

    // Input thread
    tokio::spawn(async move {
        debug!("Started input thread");
        loop {
            if event::poll(Duration::from_millis(200)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    input_tx.send(key).await.unwrap();
                }
            }
            if quit_rx.has_changed().unwrap() {
                break;
            }
        }
    });

    // Timer thread
    tokio::spawn(async move {
        debug!("Started timer thread");
        loop {
            sleep(Duration::from_millis(200)).await;
            timer_tx.send(()).await.unwrap();
            if quit_rx1.has_changed().unwrap() {
                break;
            }
        }
    });

    loop {
        // Render
        match screen_state {
            ScreenState::Main => {
                render_main(&mut terminal, &mut main_state, show_help, &config, &LOGGER)?
            }
            ScreenState::Files => {
                render_files(&mut terminal, &mut files_state, show_help, &config, &LOGGER)?
            }
            ScreenState::Frames => render_frames(
                &mut terminal,
                &mut frames_state,
                show_help,
                &config,
                &LOGGER,
            )?,
        }

        tokio::select! {
            key = input_rx.recv() => {
                let key = key.unwrap();
                match key.code {
                    KeyCode::PageUp => LOGGER.prev(),
                    KeyCode::PageDown => LOGGER.next(),
                    _ => {}
                }
                match screen_state {
                    ScreenState::Main => match main_state.handle_input(&key) {
                        AppEvent::Quit => break,
                        AppEvent::NewScreenState(s) => screen_state = s,
                        AppEvent::ToggleHelp => show_help = !show_help,
                        AppEvent::HideHelp => show_help = false,
                        _ => {}
                    },
                    ScreenState::Files => match files_state.handle_input(&key) {
                        AppEvent::Quit => break,
                        AppEvent::NewScreenState(s) => screen_state = s,
                        AppEvent::AddFiles(mut tags) => main_state.add_files(&mut tags),
                        AppEvent::ToggleHelp => show_help = !show_help,
                        AppEvent::HideHelp => show_help = false,
                        _ => {}
                    },
                    ScreenState::Frames => match frames_state.handle_input(&key) {
                        AppEvent::Quit => break,
                        AppEvent::NewScreenState(s) => screen_state = s,
                        AppEvent::ToggleHelp => show_help = !show_help,
                        AppEvent::HideHelp => show_help = false,
                        AppEvent::AddFrame(id) => main_state.add_frame(id),
                        _ => {}
                    }
                }
            }
            _ = timer_rx.recv() => { /* Nothing to do, just proceed to next loop iteration */ }
        }
    }

    quit_tx.send(()).unwrap();
    quit_tx.closed().await;

    crossterm::terminal::disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    // let logs = LOGGER.items.lock().unwrap();
    // for i in logs.iter() {
    //     println!("{:?}", i);
    // }

    Ok(())
}
