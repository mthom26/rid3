use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fs,
    path::PathBuf,
};

use crossterm::event::KeyEvent;
use id3::{frame::ExtendedText, Content, Frame, Tag, TagLike, Version};
use log::{error, info, warn};
use regex::{Match, Regex};
use tui::widgets::ListState;

use crate::{
    configuration::{actions::Action, Config},
    popups::{
        double_input::DoubleInput, help::HelpPopup, single_input::SingleInput,
        template::TemplateInput, Popup, PopupData,
    },
    state::{frame_data, update_screen_state, AppEvent, ScreenState},
    util, LOGGER,
};

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Details,
    // Edit,
    // EditInput,
}

#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub filename: String,
    pub tag: Tag,
    pub selected: bool,
}

impl Entry {
    pub fn new(path: PathBuf, tag: Tag) -> Self {
        let filename = path
            .to_str()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .to_string();

        Self {
            path,
            filename,
            tag,
            selected: false,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum DetailItem {
    FileName(String),
    Frame(Frame),
}

impl PartialOrd for DetailItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (DetailItem::FileName(_), _) => Some(Ordering::Less),
            (DetailItem::Frame(f), DetailItem::Frame(other_f)) => {
                Some(f.name().cmp(other_f.name()))
            }
            (DetailItem::Frame(_), DetailItem::FileName(_)) => Some(Ordering::Greater),
        }
    }
}

impl Ord for DetailItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (DetailItem::FileName(_), _) => Ordering::Less,
            (DetailItem::Frame(f), DetailItem::Frame(other_f)) => f.name().cmp(other_f.name()),
            (DetailItem::Frame(_), DetailItem::FileName(_)) => Ordering::Greater,
        }
    }
}

pub struct MainState {
    pub popup_stack: Vec<Box<dyn Popup>>,

    pub focus: Focus,

    pub files_state: ListState,
    pub files: Vec<Entry>,

    pub details_state: ListState,
    pub details: Vec<DetailItem>,

    help_text: Vec<String>,
    template_string: String,
    rex: Regex,
}

impl MainState {
    pub fn new() -> Self {
        let popup_stack: Vec<Box<dyn Popup>> = vec![];
        let rex = Regex::new(r"\{[\w]+\}").unwrap();

        Self {
            popup_stack,
            focus: Focus::Files,
            files_state: ListState::default(),
            files: vec![],
            details_state: ListState::default(),
            details: vec![],
            help_text: vec![],
            template_string: "{track} {title}.mp3".to_string(),
            rex,
        }
    }

    pub fn handle_input(
        &mut self,
        key: &KeyEvent,
        actions: &Vec<Action>,
        show_logs: &mut bool,
    ) -> AppEvent {
        // If the length of actions is greater than one then the KeyCode
        // pressed has been mapped to multiple actions. As long as the actions
        // have been configured properly there should only be one relevant
        // action in here so iter through actions and find it. The same applies
        // to the other screen states. This looks clumsy but works well enough
        // for now.
        let action = if actions.len() == 1 {
            actions[0]
        } else {
            let mut action = Action::None;
            // Need to check for different actions if a popup is active
            if self.popup_stack.is_empty() {
                for a in actions.iter() {
                    if *a == Action::RemoveFiles
                        || *a == Action::WriteTags
                        || *a == Action::SelectCurrent
                        || *a == Action::SelectAll
                        || *a == Action::Remove
                        || *a == Action::SpawnPopup
                        || *a == Action::TemplatePopup
                    {
                        action = *a;
                        break;
                    }
                }
            } else {
                for a in actions.iter() {
                    if *a == Action::SelectField || *a == Action::SaveChanges {
                        action = *a;
                        break;
                    }
                }
            }
            action
        };

        if let Some(popup) = self.popup_stack.last_mut() {
            match popup.handle_input(key, action) {
                AppEvent::ClosePopup => {
                    let _ = self.popup_stack.pop().unwrap();
                }
                AppEvent::ClosePopupData(data) => {
                    let _ = self.popup_stack.pop().unwrap();
                    match data {
                        PopupData::SingleInput(text) => {
                            if text.is_empty() {
                                warn!("Text frame contained an empty field, not adding new frame");
                                return AppEvent::None;
                            }

                            match self.details_state.selected() {
                                Some(i) => {
                                    match &self.details[i] {
                                        DetailItem::FileName(_) => {
                                            self.details[i] = DetailItem::FileName(text.clone());
                                            self.update_filename(text);
                                        }
                                        DetailItem::Frame(frame) => {
                                            let id = frame.id();
                                            let new_frame = Frame::text(id, text);
                                            self.details[i] = DetailItem::Frame(new_frame.clone());
                                            // Propagate frame changes to selected files
                                            self.update_files(new_frame);
                                        }
                                    }
                                }
                                None => unreachable!(),
                            }
                        }
                        PopupData::DoubleInput(description, value) => {
                            let (prev_description, id, i) =
                                if let Some(i) = self.details_state.selected() {
                                    if let DetailItem::Frame(frame) = &self.details[i] {
                                        (
                                            frame
                                                .content()
                                                .extended_text()
                                                .unwrap()
                                                .description
                                                .clone(),
                                            frame.id(),
                                            i,
                                        )
                                    } else {
                                        unreachable!()
                                    }
                                } else {
                                    unreachable!()
                                };

                            if description.is_empty() || value.is_empty() {
                                warn!("User defined text frame contained an empty field, not adding new frame");
                                return AppEvent::None;
                            }

                            let content = Content::ExtendedText(ExtendedText {
                                description: description.clone(),
                                value,
                            });
                            let new_frame = Frame::with_content(id, content);
                            self.details[i] = DetailItem::Frame(new_frame.clone());

                            // If an existing TXXX frame is edited and given a new description
                            // the old frame persists as a track can have multiple TXXX frames
                            // with unique descriptions
                            if prev_description != description {
                                self.remove_old_txxx_frame(&prev_description);
                            }
                            self.update_files(new_frame);
                        }
                        PopupData::TemplateInput(text) => {
                            if text.is_empty() {
                                warn!("Template field cannot be empty");
                                return AppEvent::None;
                            }
                            info!("Updating template string to '{}'", text);
                            self.template_string = text;
                        }
                    }
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
                Action::Help => self.spawn_help_popup(),
                Action::Prev => self.prev(),
                Action::Next => self.next(),
                Action::SwitchFocus => self.switch_focus(),
                Action::RemoveFiles => self.remove_all_files(),
                Action::WriteTags => self.write_tags().expect("Could not write tags"),
                Action::SelectCurrent => match self.focus {
                    Focus::Files => self.select_entry(),
                    _ => {}
                },
                Action::SelectAll => match self.focus {
                    Focus::Files => self.select_all_entries(),
                    _ => {}
                },
                Action::Remove => match self.focus {
                    Focus::Files => self.remove_files(),
                    Focus::Details => self.remove_frames(),
                },
                Action::SpawnPopup => self.spawn_popup(),
                Action::UpdateNames => self.update_filenames(),
                Action::TemplatePopup => self.spawn_template_popup(),
                _ => {}
            }
        }
        AppEvent::None
    }

    pub fn next(&mut self) {
        match self.focus {
            Focus::Files => {
                if self.files.is_empty() {
                    return;
                }
                let i = match self.files_state.selected() {
                    Some(i) => util::next(i, self.files.len()),
                    None => 0,
                };
                self.files_state.select(Some(i));
                self.update_details();
            }
            Focus::Details => {
                if self.details.is_empty() {
                    return;
                }
                let i = match self.details_state.selected() {
                    Some(i) => util::next(i, self.details.len()),
                    None => 0,
                };
                self.details_state.select(Some(i))
            }
        }
    }

    pub fn prev(&mut self) {
        match self.focus {
            Focus::Files => {
                if self.files.is_empty() {
                    return;
                }
                let i = match self.files_state.selected() {
                    Some(i) => util::prev(i, self.files.len()),
                    None => 0,
                };
                self.files_state.select(Some(i));
                self.update_details();
            }
            Focus::Details => {
                if self.details.is_empty() {
                    return;
                }
                let i = match self.details_state.selected() {
                    Some(i) => util::prev(i, self.details.len()),
                    None => 0,
                };
                self.details_state.select(Some(i));
            }
        }
    }

    // Add files from AppEvent::AddFiles(files)
    pub fn add_files(&mut self, files: Vec<Entry>) {
        'outer: for new_entry in files.iter() {
            for entry in self.files.iter() {
                if entry.path == new_entry.path {
                    warn!("Duplicate path");
                    continue 'outer;
                }
            }
            // TODO - remove this clone
            self.files.push(new_entry.clone());
        }
    }

    // Remove all files
    fn remove_all_files(&mut self) {
        self.files.clear();
        self.details.clear();
        self.files_state = ListState::default();
        self.details_state = ListState::default();
    }

    // Remove all selected and highlighted files
    fn remove_files(&mut self) {
        let highlighted_filename = if let Some(i) = self.files_state.selected() {
            self.files[i].filename.clone()
        } else {
            "".to_owned() // Filename will never be empty so empty string will never be matched
        };

        self.files = self
            .files
            .iter()
            .filter(|file| !file.selected)
            .filter(|file| file.filename != highlighted_filename)
            .map(|file| file.clone())
            .collect();

        if self.files.is_empty() {
            self.details.clear();
            self.files_state = ListState::default();
            self.details_state = ListState::default();
        } else {
            // Check old `files_state` isn't referring to an index outside `files` new length
            if let Some(i) = self.files_state.selected() {
                if self.files.len() < i {
                    self.files_state.select(Some(0));
                }
            }
            self.update_details();
        }
    }

    fn update_files(&mut self, new_frame: Frame) {
        for file in &mut self.files {
            if file.selected {
                file.tag.add_frame(new_frame.clone());
            }
        }
        if let Some(i) = self.files_state.selected() {
            self.files[i].tag.add_frame(new_frame);
        }
        self.update_details();
        self.next();
    }

    fn remove_old_txxx_frame(&mut self, description: &str) {
        for file in &mut self.files {
            if file.selected {
                file.tag.remove_extended_text(Some(&description), None);
            }
        }
        if let Some(i) = self.files_state.selected() {
            self.files[i]
                .tag
                .remove_extended_text(Some(&description), None);
        }
    }

    // Toggle selection of highlighted entry
    fn select_entry(&mut self) {
        match self.files_state.selected() {
            Some(i) => {
                self.files[i].selected = !self.files[i].selected;
                info!(
                    "{:?} selected: {}",
                    self.files[i].path, self.files[i].selected
                );
            }
            None => {}
        }
    }

    // Select all files in list, if all are already selected then deselect them
    fn select_all_entries(&mut self) {
        let mut any_unselected = false;
        for entry in &self.files {
            if !entry.selected {
                any_unselected = true;
                break;
            }
        }
        for entry in &mut self.files {
            entry.selected = any_unselected;
        }
        info!("All selected: {}", any_unselected);
    }

    fn update_details(&mut self) {
        let index = match self.files_state.selected() {
            Some(i) => i,
            None => {
                warn!("files_state not selected");
                return;
            }
        };

        let file_name = self.files[index].filename.clone();
        let mut new_details = vec![DetailItem::FileName(file_name)];
        for frame in self.files[index].tag.frames() {
            // Only handle text frames
            if frame.id().starts_with("T") {
                new_details.push(DetailItem::Frame(frame.clone()));
            }
        }
        // TODO - Customise this sort
        new_details.sort();
        self.details = new_details;

        // Check old `details_state` isn't referring to an index outside `new_details` length
        if let Some(i) = self.details_state.selected() {
            if self.details.len() < i {
                self.details_state.select(Some(0));
            }
        }
    }

    fn update_filename(&mut self, name: String) {
        let index = match self.files_state.selected() {
            Some(i) => i,
            None => {
                warn!("files_state not selected");
                return;
            }
        };

        self.files[index].filename = name;
    }

    fn switch_focus(&mut self) {
        match self.focus {
            Focus::Files => {
                // On switching focus to the details list for the first time
                // select the first item
                if self.details_state.selected() == None {
                    self.details_state.select(Some(0));
                }
                self.focus = Focus::Details
            }
            Focus::Details => self.focus = Focus::Files,
        }
    }

    pub fn add_frame(&mut self, id: &str) {
        info!("Adding frame {}", id);
        let frame = match id {
            "TXXX" => Frame::with_content(
                id,
                Content::ExtendedText(ExtendedText {
                    description: "Description".to_string(),
                    value: "Value".to_string(),
                }),
            ),
            _ => Frame::text(id, ""),
        };

        let highlighted = self.files_state.selected();
        for (i, entry) in self.files.iter_mut().enumerate() {
            if entry.selected || highlighted.is_some() && highlighted.unwrap() == i {
                if frame.id() != "TXXX" {
                    if entry.tag.get(frame.id()).is_none() {
                        entry.tag.add_frame(frame.clone());
                    }
                } else {
                    // Multiple TXXX frames allowed so add a new one even if one already exists
                    entry.tag.add_frame(frame.clone());
                }
            }
        }

        self.update_details();
    }

    // Remove selected frame from all selected files
    // TODO - Currently this removes all `TXXX` frames instead of just the
    //        highlighted one, need to fix this
    fn remove_frames(&mut self) {
        let id = if let Some(i) = self.details_state.selected() {
            match &self.details[i] {
                DetailItem::FileName(_) => {
                    warn!("Not a frame");
                    return;
                }
                DetailItem::Frame(frame) => frame.id(),
            }
        } else {
            unreachable!();
        };

        for file in &mut self.files {
            if file.selected {
                file.tag.remove(id);
            }
        }
        if let Some(i) = self.files_state.selected() {
            self.files[i].tag.remove(id);
        }

        self.update_details();
    }

    fn spawn_popup(&mut self) {
        if let Some(i) = self.details_state.selected() {
            match &self.details[i] {
                DetailItem::FileName(file_name) => {
                    let popup = SingleInput::new("Filename", &file_name);
                    self.popup_stack.push(Box::new(popup));
                }
                DetailItem::Frame(frame) => match frame.id() {
                    "TXXX" => {
                        let default_text = ExtendedText {
                            description: "".to_string(),
                            value: "".to_string(),
                        };
                        let text = frame.content().extended_text().unwrap_or(&default_text);

                        let popup = DoubleInput::new(&text.description, &text.value);
                        self.popup_stack.push(Box::new(popup));
                    }
                    t if t.starts_with("T") => {
                        let text = frame.content().text().expect("Could not get frame text");
                        let popup = SingleInput::new(t, text);
                        self.popup_stack.push(Box::new(popup));
                    }
                    id => warn!("Unhandled frame type: {}", id),
                },
            }
        }
    }

    fn spawn_template_popup(&mut self) {
        let popup = TemplateInput::new(&self.template_string);
        self.popup_stack.push(Box::new(popup));
    }

    // Write updated tags to files
    fn write_tags(&mut self) -> Result<(), anyhow::Error> {
        info!("Writing tags to files...");
        for entry in self.files.iter_mut() {
            entry.tag.write_to_path(&entry.path, Version::Id3v24)?;

            // Rename the file, for now the extension must be included
            // when the user enters the new filename
            let mut new_path = entry.path.clone();
            new_path.set_file_name(&entry.filename);
            fs::rename(&entry.path, &new_path)?;
            entry.path = new_path;
        }

        info!("New tags written");
        Ok(())
    }

    fn update_filenames(&mut self) {
        let mats: Vec<Match> = self.rex.find_iter(&self.template_string).collect();

        let mut frame_ids = vec![];
        for mat in &mats {
            let text: Vec<&str> = mat.as_str().split(&['{', '}']).collect();

            if let Ok(id) = frame_data::name_to_id(text[1]) {
                frame_ids.push(id);
            } else {
                warn!("Unknown frame id in template string: '{}'", text[1]);
            }
        }

        'entries: for entry in self.files.iter_mut() {
            if entry.selected {
                let mut contents = vec![];
                for id in &frame_ids {
                    if let Some(frame) = entry.tag.get(id) {
                        match frame.content() {
                            Content::Text(text) => contents.push(text),
                            _ => {
                                error!("Content type not supported");
                                continue 'entries;
                            }
                        }
                    } else {
                        error!("{} does not contain {} frame", entry.filename, id);
                        continue 'entries;
                    }
                }

                let mut new_name = self.template_string.clone();
                for (i, mat) in mats.iter().enumerate() {
                    new_name = new_name.replace(mat.as_str(), contents[i]);
                }
                entry.filename = new_name;
            }
        }

        self.update_details();
    }

    pub fn popup_widget(&self) -> Option<&Box<dyn Popup>> {
        self.popup_stack.last()
    }

    pub fn spawn_help_popup(&mut self) {
        self.popup_stack.push(Box::new(HelpPopup::new(
            "Main Help".to_owned(),
            self.help_text.clone(),
        )));
    }

    pub fn update_help_text(&mut self, config: &Config) {
        let quit = config.get_key(&Action::Quit).unwrap();
        let remove_all = config.get_key(&Action::RemoveFiles).unwrap();
        let remove = config.get_key(&Action::Remove).unwrap();
        let select_all = config.get_key(&Action::SelectAll).unwrap();
        let select = config.get_key(&Action::SelectCurrent).unwrap();
        // TODO - let update = config.get_key().unwrap();
        let write = config.get_key(&Action::WriteTags).unwrap();

        self.help_text = vec![
            format!("`{}` - Quit", util::display_keycode(quit)),
            format!("`{}` - Remove all files", util::display_keycode(remove_all)),
            format!(
                "`{}` - Remove selected files/frame",
                util::display_keycode(remove)
            ),
            format!(
                "`{}` - Select highlighted file",
                util::display_keycode(select)
            ),
            format!(
                "`{}` - Select/Deselect all files",
                util::display_keycode(select_all)
            ),
            format!("`{}` - Write changes", util::display_keycode(write)),
        ];
    }
}
