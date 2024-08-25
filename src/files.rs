use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::bytes;

pub fn list(path: &str) -> std::io::Result<Vec<File>> {
    let mut files = Vec::new();

    walk_dir(Path::new(path), &mut |file| files.push(file))?;

    files.sort_by(|a, b| b.cmp(a));

    Ok(files)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct File {
    size: u64,
    path: PathBuf,
}

impl File {
    pub fn new(path: impl Into<PathBuf>, size: u64) -> Self {
        File {
            size,
            path: path.into(),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>6}: {}",
            bytes::prettify(self.size),
            self.path.display()
        )
    }
}

fn walk_dir(path: &Path, cb: &mut impl FnMut(File)) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, cb)?;
            } else {
                cb(File::new(path, entry.metadata()?.len()));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{list, File};

    #[test]
    fn list_returns_all_files_under_the_given_path_in_descending_order() {
        let want = vec![
            File::new("./testdata/en/world.txt", 7),
            File::new("./testdata/es/mundo.txt", 6),
            File::new("./testdata/en/hello.txt", 6),
            File::new("./testdata/es/hola.txt", 5),
        ];
        let got = list("./testdata").unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn file_can_be_formatted_as_a_string() {
        struct Case {
            file: File,
            want: &'static str,
        }
        let cases = vec![
            Case {
                file: File::new("/path/to/file.txt", 1000),
                want: "  1 KB: /path/to/file.txt",
            },
            Case {
                file: File::new("/path/to/file.txt", 34250),
                want: " 34 KB: /path/to/file.txt",
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
}
