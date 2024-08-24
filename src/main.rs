use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, default_value_t = 5)]
    number: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let n = args.number;
    let entries = dstats::list_entries(&args.path)?;

    println!("*** Top {n} largest files ***");
    for entry in entries.iter().take(n) {
        println!("{entry}");
    }

    Ok(())
}
