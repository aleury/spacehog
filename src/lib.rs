mod files;

use std::io;

use files::File;

/// Returns the top `n` largest files under the provided path.
///
/// # Errors
///
/// Returns an I/O error if unable to scan the provided path.
pub fn find_top_n_largest_files(path: &str, n: usize) -> io::Result<Vec<File>> {
    let mut results = Vec::with_capacity(n);

    for file in files::from_path(path)? {
        if let Some(smallest_file) = results.last() {
            if results.len() < n || file > *smallest_file {
                results.push(file);
                results.sort_by(|a, b| b.cmp(a));
                if results.len() > n {
                    results.pop();
                }
            }
        } else {
            results.push(file);
        }
    }

    Ok(results)
}
