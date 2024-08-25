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

    println!("*** Top {} largest files ***", args.number);
    for file in dstats::find_top_n_largest_files(&args.path, args.number)? {
        println!("{file}");
    }

    Ok(())
}
