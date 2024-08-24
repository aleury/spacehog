fn main() -> std::io::Result<()> {
    let n = 5;
    let entries = dstats::list_entries(".")?;

    println!("*** Top {n} largest files ***");
    for entry in entries.iter().take(n) {
        println!("{entry}");
    }

    Ok(())
}
