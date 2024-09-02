mod bytes;
mod files;

use std::{collections::BTreeMap, io};

use files::File;

/// Returns the top `n` largest files under the provided path.
///
/// # Errors
///
/// Returns an I/O error if unable to scan the provided path.
pub fn find_top_n_largest_files(path: &str, n: usize) -> io::Result<Vec<File>> {
    let mut results = BTreeMap::new();
    for file in files::from_path(path)? {
        results.insert(file.clone(), file);
    }
    Ok(results.into_values().rev().take(n).collect())
}
