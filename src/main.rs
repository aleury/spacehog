#![allow(clippy::cast_possible_truncation)]
use clap::Parser;
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::Write;

use spacehog::get_files_with_minimum_size;

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
    let mut count = 0;
    let mut stdout = std::io::stdout();
    let rx = get_files_with_minimum_size(&args.path, args.number, args.minsize)?;
    while let Ok(results) = rx.recv() {
        count = results.len();
        stdout.queue(cursor::SavePosition)?;
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        if !results.is_empty() {
            let mut buf = Vec::new();
            writeln!(&mut buf, "*** Top {count} largest files ***")?;
            for (size, path) in results {
                writeln!(&mut buf, "{} {}", size, path.display())?;
            }
            stdout.write_all(&buf)?;
        }

        stdout.execute(cursor::RestorePosition)?;
        stdout.flush()?;
    }
    if count > 0 {
        stdout.execute(cursor::MoveDown((count + 1) as u16))?;
    } else {
        println!("No files found.");
    }
    Ok(())
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
