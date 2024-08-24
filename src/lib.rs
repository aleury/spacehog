use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

pub fn run(path: &str, n: usize) -> std::io::Result<()> {
    let entries = list_entries(path)?;
    println!("*** Top {n} largest files ***");
    for entry in entries.iter().take(n) {
        println!("{entry}");
    }
    Ok(())
}

fn list_entries(path: &str) -> std::io::Result<Vec<Entry>> {
    let mut entries = Vec::new();

    walk_dir(Path::new(path), &mut |entry| entries.push(entry))?;

    entries.sort_by(|a, b| b.cmp(a));

    Ok(entries)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Entry {
    size: u64,
    path: PathBuf,
}

impl Entry {
    pub fn new(path: impl Into<PathBuf>, size: u64) -> Self {
        Entry {
            size,
            path: path.into(),
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>6}: {}",
            prettify_bytes(self.size),
            self.path.display()
        )
    }
}

fn walk_dir(path: &Path, cb: &mut impl FnMut(Entry)) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, cb)?;
            } else {
                cb(Entry::new(entry.path(), entry.metadata()?.len()));
            }
        }
    }
    Ok(())
}

fn prettify_bytes(n: u64) -> String {
    let units = ["B", "Kb", "Mb", "Gb", "Tb"];

    let base = (n as f64).ln() / 1000f64.ln();
    let value = 1000f64.powf(base - base.floor());
    let suffix = units[base.floor() as usize];

    format!("{value:.0} {suffix}")
}

#[cfg(test)]
mod test {
    use super::{list_entries, prettify_bytes, Entry};

    #[test]
    fn list_entries_returns_all_files_under_the_given_path_in_descending_order() {
        let want = vec![
            Entry::new("./testdata/en/world.txt", 7),
            Entry::new("./testdata/es/mundo.txt", 6),
            Entry::new("./testdata/en/hello.txt", 6),
            Entry::new("./testdata/es/hola.txt", 5),
        ];
        let got = list_entries("./testdata").unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn entry_can_be_formatted_as_a_string() {
        struct Case {
            entry: Entry,
            want: &'static str,
        }
        let cases = vec![
            Case {
                entry: Entry::new("/path/to/file.txt", 1000),
                want: "  1 Kb: /path/to/file.txt",
            },
            Case {
                entry: Entry::new("/path/to/file.txt", 34250),
                want: " 34 Kb: /path/to/file.txt",
            },
        ];
        for case in cases {
            let got = case.entry.to_string();
            assert_eq!(case.want, got);
        }
    }

    #[test]
    fn entries_can_be_sorted_in_descending_order() {
        let mut entries = vec![
            Entry::new("/path/to/a.txt", 20),
            Entry::new("/path/to/b.txt", 10),
            Entry::new("/path/to/d.txt", 10),
            Entry::new("/path/to/c.txt", 30),
        ];

        let want = vec![
            Entry::new("/path/to/c.txt", 30),
            Entry::new("/path/to/a.txt", 20),
            Entry::new("/path/to/d.txt", 10),
            Entry::new("/path/to/b.txt", 10),
        ];
        entries.sort_by(|a, b| b.cmp(a));

        assert_eq!(want, entries);
    }

    #[test]
    fn prettify_bytes_formats_a_number_of_bytes_as_human_readable_text() {
        struct Case {
            size: u64,
            want: &'static str,
        }
        let cases = vec![
            Case {
                size: 525,
                want: "525 B",
            },
            Case {
                size: 1000,
                want: "1 Kb",
            },
            Case {
                size: 1024,
                want: "1 Kb",
            },
            Case {
                size: 2024,
                want: "2 Kb",
            },
            Case {
                size: 2_000_000,
                want: "2 Mb",
            },
            Case {
                size: 5_500_250,
                want: "6 Mb",
            },
            Case {
                size: 1_000_000_000,
                want: "1 Gb",
            },
            Case {
                size: 25_000_000_000,
                want: "25 Gb",
            },
            Case {
                size: 3_000_000_000_000,
                want: "3 Tb",
            },
        ];
        for case in cases {
            let got = prettify_bytes(case.size);
            assert_eq!(case.want.to_string(), got);
        }
    }
}
