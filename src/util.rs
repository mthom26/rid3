use crossterm::event::KeyCode;

// Convenience function to get next element in a Vec of length `len`, current index `i`
// while wrapping around.
pub fn next(i: usize, len: usize) -> usize {
    if i >= len - 1 {
        0
    } else {
        i + 1
    }
}

// Convenience function to get previous element in a Vec of length `len`, current index `i`
// while wrapping around.
pub fn prev(i: usize, len: usize) -> usize {
    if i <= 0 {
        len - 1
    } else {
        i - 1
    }
}

// Nicely display KeyCode
pub fn display_keycode(keycode: &KeyCode) -> String {
    match keycode {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Up => "Up".to_owned(),
        KeyCode::Down => "Down".to_owned(),
        KeyCode::Left => "Left".to_owned(),
        KeyCode::Right => "Right".to_owned(),
        KeyCode::Esc => "Esc".to_owned(),
        KeyCode::Tab => "Tab".to_owned(),
        KeyCode::Backspace => "Backspace".to_owned(),
        KeyCode::Enter => "Enter".to_owned(),
        KeyCode::Home => "Home".to_owned(),
        KeyCode::End => "End".to_owned(),
        KeyCode::PageUp => "PageUp".to_owned(),
        KeyCode::PageDown => "PageDown".to_owned(),
        KeyCode::BackTab => "BackTab".to_owned(),
        KeyCode::Delete => "Delete".to_owned(),
        KeyCode::Insert => "Insert".to_owned(),
        _ => todo!(),
    }
}
