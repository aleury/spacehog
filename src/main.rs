#![allow(clippy::cast_possible_truncation)]
use clap::Parser;
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::{
    io::{self, Write},
    path::PathBuf,
};

use spacehog::{get_files_with_minimum_size, FileSize};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, default_value_t = 5)]
    number: usize,

    #[arg(short, default_value_t = 0)]
    minsize: u64,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut stdout = std::io::stdout();
    let mut app = App::new(&mut stdout);

    let rx = get_files_with_minimum_size(&args.path, args.number, args.minsize)?;
    while let Ok(results) = rx.recv() {
        if !results.is_empty() {
            app.update(results);
            app.render()?;
        }
    }

    Ok(())
}

struct App<'a, Out: Write> {
    out: &'a mut Out,
    files: Vec<(FileSize, PathBuf)>,
}

impl<'a, Out: Write> App<'a, Out> {
    fn new(out: &'a mut Out) -> Self {
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
        self.out
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        let mut buf = Vec::new();
        writeln!(&mut buf, "*** Top {} largest files ***", self.files.len())?;
        for (size, path) in &self.files {
            writeln!(&mut buf, "{} {}", size, path.display())?;
        }
        self.out.write_all(&buf)?;

        self.out.queue(cursor::RestorePosition)?;
        self.out.flush()
    }
}

impl<'a, Out: Write> Drop for App<'a, Out> {
    fn drop(&mut self) {
        if self.files.is_empty() {
            println!("No files found.");
        } else {
            let _ = self
                .out
                .execute(cursor::MoveDown((self.files.len() + 1) as u16));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

    #[test]
    fn cli_args_can_be_parsed_without_panicing() {
        Args::parse();
    }
}
