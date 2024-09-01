use std::fmt::Display;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;

use humanize_bytes::humanize_bytes_decimal;

pub fn from_path(path: &str) -> io::Result<FileIter> {
    let dir = std::fs::read_dir(path)?;
    Ok(FileIter { stack: vec![dir] })
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct File {
    size: Size,
    path: PathBuf,
}

impl File {
    pub fn new(path: impl Into<PathBuf>, size: u64) -> Self {
        File {
            size: Size(size),
            path: path.into(),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.size, self.path.display())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Size(u64);

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>8}", humanize_bytes_decimal!(self.0))
    }
}

pub struct FileIter {
    stack: Vec<ReadDir>,
}

impl Iterator for FileIter {
    type Item = File;

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
                        return Some(File::new(path, entry.metadata().ok()?.len()));
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
mod test {
    use super::{from_path, File};

    #[test]
    fn file_can_be_formatted_as_a_string() {
        struct Case {
            file: File,
            want: &'static str,
        }
        let cases = vec![
            Case {
                file: File::new("/path/to/file.txt", 1000),
                want: "    1 kB: /path/to/file.txt",
            },
            Case {
                file: File::new("/path/to/file.txt", 34250),
                want: " 34.2 kB: /path/to/file.txt",
            },
        ];
        for case in cases {
            let got = case.file.to_string();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn files_can_be_sorted_in_descending_order() {
        let mut files = vec![
            File::new("/path/to/a.txt", 20),
            File::new("/path/to/b.txt", 10),
            File::new("/path/to/d.txt", 10),
            File::new("/path/to/c.txt", 30),
        ];

        let want = vec![
            File::new("/path/to/c.txt", 30),
            File::new("/path/to/a.txt", 20),
            File::new("/path/to/d.txt", 10),
            File::new("/path/to/b.txt", 10),
        ];
        files.sort_by(|a, b| b.cmp(a));

        assert_eq!(want, files);
    }

    #[test]
    fn from_path_returns_iterator_over_files_in_the_given_path() {
        let want = vec![
            File::new("./testdata/en/hello.txt", 6),
            File::new("./testdata/en/world.txt", 7),
            File::new("./testdata/es/hola.txt", 5),
            File::new("./testdata/es/mundo.txt", 6),
        ];
        let mut got = from_path("./testdata").unwrap().collect::<Vec<_>>();
        // Sort by path for consistent ordering.
        got.sort_by_key(|f| f.path.clone());
        assert_eq!(want, got);
    }
}
