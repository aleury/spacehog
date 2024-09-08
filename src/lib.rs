//! Find large files on your system.
mod bytes;

use std::{
    collections::BTreeMap, fmt::Display, fs::ReadDir, io, path::PathBuf, sync::mpsc, thread,
    time::Instant,
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
/// let results = find_top_n_largest_files("testdata", 5).unwrap();
///
/// assert_eq!(results.len(), 4);
/// ```
pub fn find_top_n_largest_files(path: &str, n: usize) -> io::Result<Vec<(FileSize, PathBuf)>> {
    let mut results = BTreeMap::new();
    for entry in find_files_in_path(path)? {
        results.insert(entry.clone(), entry);
    }
    Ok(results.into_values().rev().take(n).collect())
}

#[must_use]
pub fn stream_files_larger_than_min_size(
    path: &str,
    limit: usize,
    minimum: FileSize,
) -> mpsc::Receiver<Vec<(FileSize, PathBuf)>> {
    let (tx, rx) = mpsc::sync_channel(10);

    let path = path.to_string();
    thread::spawn(move || {
        let mut timer = Instant::now();
        let mut results = BTreeMap::new();
        for entry in find_files_in_path(&path).unwrap_or_default() {
            if entry.0 >= minimum {
                results.insert(entry.clone(), entry);
            }
            if timer.elapsed().as_millis() > 16 {
                let snapshot: Vec<_> = results.clone().into_values().rev().take(limit).collect();
                if let Err(e) = tx.send(snapshot) {
                    println!("failed to send entry: {e:?}");
                };
                timer = Instant::now();
            }
        }
        drop(tx);
    });

    rx
}

/// The size of a file in bytes.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileSize(u64);

impl From<u64> for FileSize {
    fn from(value: u64) -> Self {
        FileSize(value)
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bytes::humanize(self.0))
    }
}

fn find_files_in_path(path: &str) -> io::Result<FileIter> {
    let dir = std::fs::read_dir(path)?;
    Ok(FileIter { stack: vec![dir] })
}

#[derive(Default)]
struct FileIter {
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
