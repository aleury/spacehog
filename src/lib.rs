//! Find large files on your system.
mod bytes;
mod files;

use std::{collections::BTreeMap, io};

use files::File;

/// Returns the top `n` largest files under the provided path.
///
/// # Errors
///
/// Returns an I/O error if unable to scan the provided path.
///
/// # Examples
///
/// ```
/// # use spacehog::find_top_n_largest_files;
/// # fn main() {
/// let result = find_top_n_largest_files("testdata", 5).unwrap();
///
/// assert_eq!(result.len(), 4);
/// # }
/// ```
pub fn find_top_n_largest_files(path: &str, n: usize) -> io::Result<Vec<File>> {
    let mut results = BTreeMap::new();
    for file in files::from_path(path)? {
        results.insert(file.clone(), file);
    }
    Ok(results.into_values().rev().take(n).collect())
}
