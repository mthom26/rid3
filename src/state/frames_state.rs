use crossterm::event::KeyEvent;
use tui::widgets::ListState;

use crate::{
    configuration::{actions::Action, Config},
    popups::{help::HelpPopup, Popup},
    state::{frame_data::SUPPORTED_FRAMES, update_screen_state, AppEvent, ScreenState},
    util, LOGGER,
};

pub struct FramesState {
    pub popup_stack: Vec<Box<dyn Popup>>,

    pub frames_state: ListState,

    help_text: Vec<String>,
}

impl FramesState {
    pub fn new() -> Self {
        let mut frames_state = ListState::default();
        frames_state.select(Some(0));

        Self {
            popup_stack: vec![],
            frames_state,
            help_text: vec![],
        }
    }

    pub fn handle_input(
        &mut self,
        key: &KeyEvent,
        actions: &Vec<Action>,
        show_logs: &mut bool,
    ) -> AppEvent {
        let action = if actions.len() == 1 {
            actions[0]
        } else {
            let mut action = Action::None;
            for a in actions.iter() {
                if *a == Action::AddFrame {
                    action = *a;
                    break;
                }
            }
            action
        };

        if let Some(popup) = self.popup_stack.last_mut() {
            match popup.handle_input(key, action) {
                AppEvent::ClosePopup => {
                    // Need to return relevant data from popup here, probably another enum...
                    let _p = self.popup_stack.pop().unwrap();
                }
                AppEvent::SwitchScreen(s) => return update_screen_state(s),
                _ => {}
            }
        } else {
            match action {
                Action::Quit => return AppEvent::Quit,
                Action::ScreenOne => return update_screen_state(ScreenState::Main),
                Action::ScreenTwo => return update_screen_state(ScreenState::Files),
                Action::ScreenThree => return update_screen_state(ScreenState::Frames),
                Action::ToggleLogs => *show_logs = !*show_logs,
                Action::LogsPrev => LOGGER.prev(),
                Action::LogsNext => LOGGER.next(),
                Action::Help => self.spawn_help_popup(HelpPopup::new(
                    "Frames Help".to_owned(),
                    self.help_text.clone(),
                )),
                Action::Prev => self.prev(),
                Action::Next => self.next(),
                Action::AddFrame => return AppEvent::AddFrame(self.frame_id()),
                _ => {}
            }
        }
        AppEvent::None
    }

    pub fn next(&mut self) {
        let i = match self.frames_state.selected() {
            Some(i) => util::next(i, SUPPORTED_FRAMES.len()),
            None => 0,
        };
        self.frames_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.frames_state.selected() {
            Some(i) => util::prev(i, SUPPORTED_FRAMES.len()),
            None => 0,
        };
        self.frames_state.select(Some(i));
    }

    fn frame_id(&self) -> &'static str {
        match self.frames_state.selected() {
            Some(i) => SUPPORTED_FRAMES[i].id,
            None => unreachable!(),
        }
    }

    pub fn popup_widget(&self) -> Option<&Box<dyn Popup>> {
        self.popup_stack.last()
    }

    pub fn spawn_help_popup(&mut self, help_popup: HelpPopup) {
        self.popup_stack.push(Box::new(help_popup));
    }

    pub fn update_help_text(&mut self, config: &Config) {
        let quit = config.get_key(&Action::Quit).unwrap();
        let add = config.get_key(&Action::AddFrame).unwrap();

        self.help_text = vec![
            format!("`{}` - Quit", util::display_keycode(quit)),
            format!("`{}` - Add selected frame", util::display_keycode(add)),
        ];
    }
}
