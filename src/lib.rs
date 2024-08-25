mod bytes;
mod files;

use std::io;

use files::File;

/// Returns the top `n` largest files under the provided path.
///
/// # Errors
///
/// Returns an I/O error if unable to scan the provided path.
pub fn find_top_n_largest_files(path: &str, n: usize) -> io::Result<Vec<File>> {
    Ok(files::list(path)?.into_iter().take(n).collect())
}
