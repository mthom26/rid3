use std::{cmp::Ordering, fs::DirEntry};

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

// Sort a list of `DirEntry`, directories first then by filename
pub fn sort_files(files: &mut Vec<DirEntry>) {
    files.sort_by(|a, b| {
        match (
            a.file_type().unwrap().is_dir(),
            b.file_type().unwrap().is_dir(),
        ) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (_, _) => a
                .file_name()
                .to_ascii_lowercase()
                .cmp(&b.file_name().to_ascii_lowercase()),
        }
    });
}
