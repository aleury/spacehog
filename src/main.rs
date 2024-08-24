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
    dstats::run(&args.path, args.number)
}
