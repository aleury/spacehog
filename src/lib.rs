//! Find large files on your system.
mod bytes;

use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::ReadDir,
    io,
    path::{Path, PathBuf},
};

/// Returns the top `n` largest files under the provided path.
///
/// # Errors
///
/// Returns an I/O error if unable to scan the provided path.
///
/// # Examples
///
/// ```
/// use spacehog::find_top_n_largest_files;
///
/// let results = find_top_n_largest_files("testdata", 5, false).unwrap();
///
/// assert_eq!(results.len(), 4);
/// ```
pub fn find_top_n_largest_files(
    path: &str,
    n: usize,
    ignore_hidden: bool,
) -> io::Result<Vec<(FileSize, PathBuf)>> {
    let mut results = BTreeMap::new();
    for entry in find_files_in_path(path, ignore_hidden)? {
        results.insert(entry.clone(), entry);
    }
    Ok(results.into_values().rev().take(n).collect())
}

/// The size of a file in bytes.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileSize(u64);

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bytes::humanize(self.0))
    }
}

fn find_files_in_path(path: &str, ignore_hidden: bool) -> io::Result<FileIter> {
    let dir = std::fs::read_dir(path)?;
    Ok(FileIter {
        ignore_hidden,
        stack: vec![dir],
    })
}

struct FileIter {
    ignore_hidden: bool,
    stack: Vec<ReadDir>,
}

impl Iterator for FileIter {
    type Item = (FileSize, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let dir = self.stack.last_mut()?;
            if let Some(entry) = dir.next() {
                let entry = entry.ok()?;
                let path = entry.path();
                if self.ignore_hidden && is_hidden_path(&path) {
                    continue;
                }
                if path.is_dir() {
                    self.stack.push(std::fs::read_dir(path).ok()?);
                } else {
                    let size = entry.metadata().ok()?.len();
                    return Some((FileSize(size), path));
                }
            } else {
                self.stack.pop();
            }
        }
    }
}

fn is_hidden_path<P: AsRef<Path>>(path: P) -> bool {
    if let Some(name) = path.as_ref().file_name() {
        name.to_str().map_or(false, |s| s.starts_with('.'))
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::FileSize;

    #[test]
    fn file_sizes_can_be_formatted_as_a_string() {
        struct Case {
            file: FileSize,
            want: &'static str,
        }
        let cases = vec![
            Case {
                file: FileSize(1000),
                want: "1 KB",
            },
            Case {
                file: FileSize(34_250),
                want: "34 KB",
            },
        ];
        for case in cases {
            let got = case.file.to_string();
            assert_eq!(case.want, got);
        }
    }
}
