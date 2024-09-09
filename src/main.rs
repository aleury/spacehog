#![allow(clippy::cast_possible_truncation)]
use clap::Parser;
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::Write;

use spacehog::stream_files_larger_than_min_size;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, default_value_t = 5)]
    number: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut stdout = std::io::stdout();
    let stream = stream_files_larger_than_min_size(&args.path, args.number, 1_000_000.into())?;
    while let Ok(results) = stream.recv() {
        stdout.queue(cursor::SavePosition)?;
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        let mut buf = Vec::new();
        writeln!(&mut buf, "*** Top {} largest files ***", results.len())?;
        for (size, path) in results {
            writeln!(&mut buf, "{} {}", size, path.display())?;
        }
        stdout.write_all(&buf)?;

        stdout.execute(cursor::RestorePosition)?;
        stdout.flush()?;
    }
    stdout.execute(cursor::MoveDown((args.number + 1) as u16))?;
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
