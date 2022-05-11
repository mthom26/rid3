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
