//! Find large files on your system.
mod bytes;

use std::{collections::BTreeMap, fmt::Display, fs::ReadDir, io, path::PathBuf};

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
/// let results = find_top_n_largest_files("testdata", 5).unwrap();
///
/// assert_eq!(results.len(), 4);
/// ```
pub fn find_top_n_largest_files(path: &str, n: usize) -> io::Result<Vec<(FileSize, PathBuf)>> {
    let mut results = BTreeMap::new();
    for entry in walk_dir(path)? {
        results.insert(entry.clone(), entry);
    }
    Ok(results.into_values().rev().take(n).collect())
}

/// The size of a file in bytes.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileSize(u64);

impl From<FileSize> for u64 {
    fn from(size: FileSize) -> Self {
        size.0
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bytes::humanize(self.0))
    }
}

fn walk_dir(path: &str) -> io::Result<FileIter> {
    let dir = std::fs::read_dir(path)?;
    Ok(FileIter { stack: vec![dir] })
}

struct FileIter {
    stack: Vec<ReadDir>,
}

impl Iterator for FileIter {
    type Item = (FileSize, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(dir) = self.stack.last_mut() {
                // Explore the current directory.
                if let Some(entry) = dir.next() {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.is_dir() {
                        self.stack.push(std::fs::read_dir(path).ok()?);
                    } else {
                        let size = entry.metadata().ok()?.len();
                        return Some((FileSize(size), path));
                    }
                } else {
                    // No more entries in the current directory.
                    self.stack.pop();
                }
            } else {
                // No more directories to explore.
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::FileSize;

    #[test]
    fn file_size_can_convert_into_a_u64() {
        let size = FileSize(100);
        assert_eq!(u64::from(size), 100u64);
    }

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
