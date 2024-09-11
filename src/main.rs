#![allow(clippy::cast_possible_truncation)]
use clap::Parser;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::{
    io::{self, Write},
    path::PathBuf,
};

use spacehog::{find_top_n_largest_files, FileSize};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, default_value_t = 5)]
    number: usize,

    #[arg(long, default_value_t = false)]
    hidden: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let ignore_hidden = !args.hidden;
    let mut app = App::new(std::io::stdout());

    let rx = find_top_n_largest_files(&args.path, args.number, ignore_hidden)?;
    while let Ok(results) = rx.recv() {
        if !results.is_empty() {
            app.update(results);
            app.render()?;
        }
    }
    app.close()?;

    Ok(())
}

struct App<Out: Write> {
    out: Out,
    files: Vec<(FileSize, PathBuf)>,
}

impl<Out: Write> App<Out> {
    fn new(out: Out) -> Self {
        Self {
            out,
            files: Vec::new(),
        }
    }

    fn update(&mut self, files: Vec<(FileSize, PathBuf)>) {
        self.files = files;
    }

    fn render(&mut self) -> io::Result<()> {
        self.out.queue(cursor::SavePosition)?;
        self.out.queue(terminal::Clear(ClearType::FromCursorDown))?;

        let mut buf = Vec::new();
        writeln!(&mut buf, "*** Top {} largest files ***", self.files.len())?;
        for (size, path) in &self.files {
            writeln!(&mut buf, "{} {}", size, path.display())?;
        }
        self.out.write_all(&buf)?;

        self.out.queue(cursor::RestorePosition)?;
        self.out.flush()
    }

    fn close(&mut self) -> io::Result<()> {
        if self.files.is_empty() {
            writeln!(self.out, "No files found.")?;
        } else {
            let row = self.files.len() + 1;
            self.out.execute(cursor::MoveDown(row as u16))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{App, Args};
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn cli_args_can_be_parsed_without_panicing() {
        Args::parse();
    }

    #[test]
    fn test_close_with_no_files() {
        let mut output = Vec::new();
        let mut app = App::new(&mut output);

        app.close().unwrap();

        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str, "No files found.\n");
    }

    #[test]
    fn test_close_with_files_moves_cursor_to_row_after_last_file() {
        let mut output = Vec::new();
        let mut app = App::new(&mut output);

        app.update(vec![
            (2048.into(), PathBuf::from("file1.txt")),
            (1024.into(), PathBuf::from("file2.txt")),
        ]);
        app.close().unwrap();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.ends_with("\x1B[3B"));
    }
}
