//! File read and write helper functions.

use std::io;
use std::path::Path;

/// Read whitespace-separated values from a file.
pub(crate) fn read_file<P: AsRef<Path>>(p: P) -> io::Result<Vec<String>> {
    fs_err::read_to_string(p).map(|x| x.split_whitespace().map(|s| s.to_string()).collect())
}
