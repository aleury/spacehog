//! Find large files on your system.
mod bytes;

use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs::ReadDir;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Instant;

/// Stream the top `n` largest files under the provided path.
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
/// let rx = find_top_n_largest_files("testdata", 5, true).unwrap();
///
/// let results = rx.recv().unwrap();
///
/// assert_eq!(results.len(), 4);
/// ```
pub fn find_top_n_largest_files(
    path: &str,
    limit: usize,
    ignore_hidden: bool,
) -> io::Result<mpsc::Receiver<Vec<(FileSize, PathBuf)>>> {
    let path = path.to_string();
    let (tx, rx) = mpsc::channel();
    let file_iter = find_files_in_path(&path, ignore_hidden)?;

    std::thread::spawn(move || {
        let mut timer = Instant::now();
        let mut results = BTreeMap::new();
        for entry in file_iter {
            results.insert(entry.clone(), entry);
            if timer.elapsed().as_millis() >= 16 {
                send_snapshot(&tx, &results, limit);
                timer = Instant::now();
            }
        }
        send_snapshot(&tx, &results, limit);
    });

    Ok(rx)
}

fn send_snapshot(
    tx: &mpsc::Sender<Vec<(FileSize, PathBuf)>>,
    results: &BTreeMap<(FileSize, PathBuf), (FileSize, PathBuf)>,
    limit: usize,
) {
    let snapshot = results.values().rev().take(limit).cloned().collect();
    if let Err(e) = tx.send(snapshot) {
        eprintln!("failed to send entry: {e:?}");
    };
}

/// The size of a file in bytes.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileSize(u64);

impl From<u64> for FileSize {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

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

#[derive(Default)]
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
