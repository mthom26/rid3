use std::path::PathBuf;

use id3::Tag;

use crate::state::main_state::Entry;

pub async fn get_id3s() -> Result<Vec<Entry>, anyhow::Error> {
    let tags = [
        "test-files/test.mp3",
        "test-files/test2.mp3",
        // "test-files/test3.mp3",
    ]
    .iter()
    .map(|p| {
        Entry::new(
            PathBuf::from(p),
            Tag::read_from_path(p).expect("Could not read Tag"),
        )
    })
    .collect();

    Ok(tags)
}

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
