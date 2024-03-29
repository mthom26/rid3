use std::{env, io, path::PathBuf, sync::Mutex, time::Duration};

use crossterm::{
    event::{self, Event},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures;
use log::{debug, warn, LevelFilter};
use notify::{self, event::ModifyKind, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{
    sync::{mpsc, watch},
    time::sleep,
};
use tui::{backend::CrosstermBackend, Terminal};

mod args;
mod configuration;
mod logger;
mod popups;
mod render;
mod state;
mod util;
use args::get_args;
use configuration::{get_config_dir, Config};
use logger::Logger;
use render::{files_render::files_render, frames_render::frames_render, main_render::main_render};
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

    let mut app_config = Config::new();

    let mut screen_state = ScreenState::Main;
    let mut main_state = MainState::new(app_config.get_template_string());
    let mut files_state = FilesState::new(dir)?;
    let mut frames_state = FramesState::new();
    main_state.update_help_text(&app_config);
    files_state.update_help_text(&app_config);
    frames_state.update_help_text(&app_config);

    let mut show_logs = true;
    let mut update_config = false;

    let (input_tx, mut input_rx) = mpsc::channel(32);
    let (timer_tx, mut timer_rx) = mpsc::channel(32);
    let (config_tx, mut config_rx) = tokio::sync::mpsc::channel(32);
    let (quit_tx, quit_rx) = watch::channel(());
    let (quit_rx1, quit_rx2) = (quit_rx.clone(), quit_rx.clone());

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

    // Watch for changes in config file
    tokio::spawn(async move {
        if let Some(path) = get_config_dir() {
            let mut watcher = RecommendedWatcher::new(
                move |res| {
                    futures::executor::block_on(async {
                        config_tx.send(res).await.unwrap();
                    })
                },
                notify::Config::default(),
            )
            .unwrap();

            watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

            // These do not work???
            // let watcher = RecommendedWatcher::new(
            //     move |res| {
            //         futures::executor::block_on(async {
            //             config_tx.send(res).await.unwrap();
            //         })
            //     },
            //     notify::Config::default(),
            // );

            // if watcher.is_ok() {
            //     watcher.unwrap().watch(&path, RecursiveMode::NonRecursive).unwrap();
            // }

            // match RecommendedWatcher::new(
            //     move |res| {
            //         futures::executor::block_on(async {
            //             config_tx.send(res).await.unwrap();
            //         })
            //     },
            //     notify::Config::default(),
            // ) {
            //     Ok(mut watcher) => {
            //         watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();
            //         error!("Created watcher.");
            //     }
            //     Err(_) => {
            //         error!("Could not create watcher.");
            //     }
            // }

            loop {
                sleep(Duration::from_millis(200)).await;
                if quit_rx2.has_changed().unwrap() {
                    break;
                }
            }
        } else {
            warn!("Could not find user config directory. Not watching for changes.");
        }
    });

    loop {
        // Render
        match screen_state {
            ScreenState::Main => {
                main_render(
                    &mut terminal,
                    &LOGGER,
                    &app_config,
                    show_logs,
                    &mut main_state,
                )?;
            }
            ScreenState::Files => {
                files_render(
                    &mut terminal,
                    &LOGGER,
                    &app_config,
                    show_logs,
                    &mut files_state,
                )?;
            }
            ScreenState::Frames => {
                frames_render(
                    &mut terminal,
                    &LOGGER,
                    &app_config,
                    show_logs,
                    &mut frames_state,
                )?;
            }
        }

        tokio::select! {
            key = input_rx.recv() => {
                let key = key.unwrap();
                let actions = if let Some(actions) = app_config.get_actions(&key.code) {
                    actions.clone()
                } else {
                    // warn!("Keycode {:?} not assigned to any action", key.code);
                    vec![]
                };

                match screen_state {
                    ScreenState::Main => match main_state.handle_input(&key, &actions, &mut show_logs) {
                        AppEvent::Quit => break,
                        AppEvent::UpdateConfig => update_config = true,
                        AppEvent::SwitchScreen(s) => screen_state = s,
                        _ => {}
                    }
                    ScreenState::Files => match files_state.handle_input(&key, &actions, &mut show_logs){
                        AppEvent::Quit => break,
                        AppEvent::SwitchScreen(s) => screen_state = s,
                        AppEvent::AddFiles(files) => main_state.add_files(files),
                        _ => {}
                    }
                    ScreenState::Frames => match frames_state.handle_input(&key, &actions, &mut show_logs) {
                        AppEvent::Quit => break,
                        AppEvent::SwitchScreen(s) => screen_state = s,
                        AppEvent::AddFrame(frame_id) => main_state.add_frame(frame_id),
                        _ => {}
                    }
                }
            }
            watcher_event = config_rx.recv() => {
                if let Some(event) = watcher_event {
                    if let Ok(e) = event {
                        // Different text editors emit many different events when a file is updated
                        // so matching for the `Data()` ModifyKind here should prevent most unnecessary
                        // config rebuilds
                        //
                        // TODO - When using some editors the new config is rebuilt too quickly (before
                        // the new data has actually been written to file). Need to fix this...
                        match e.kind {
                            EventKind::Modify(ModifyKind::Data(_)) => {
                                app_config = Config::new();
                                main_state.update_help_text(&app_config);
                                files_state.update_help_text(&app_config);
                                frames_state.update_help_text(&app_config);
                            },
                            _ => {}
                        }
                    }
                }
            }
            _ = timer_rx.recv() => { /* Nothing to do, just proceed to next loop iteration */ }
        }
    }

    quit_tx.send(()).unwrap();
    quit_tx.closed().await;

    if update_config {
        configuration::update_config(&main_state.template_string)?;
    }

    crossterm::terminal::disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;

    // dbg!("{}", app_config);
    // let logs = LOGGER.items.lock().unwrap();
    // for i in logs.iter() {
    //     println!("{:?}", i);
    // }

    Ok(())
}
