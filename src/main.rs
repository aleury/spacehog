use clap::Parser;
use spinoff::{spinners, Color, Spinner};

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, default_value_t = 5)]
    number: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut sp = Spinner::new(spinners::Dots, "Scanning files...", Color::Blue);

    let results = dstats::find_top_n_largest_files(&args.path, args.number)?;
    sp.clear();

    println!("*** Top {} largest files ***", args.number);
    for file in results {
        println!("{file}");
    }

    Ok(())
}
