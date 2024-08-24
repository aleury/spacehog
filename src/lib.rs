mod bytes;
mod entries;

/// Prints the top `n` largest files under the provided path.
///
/// # Errors
///
/// Will return the I/O error if unable to scan the provided path.
pub fn display_largest_files(path: &str, n: usize) -> std::io::Result<()> {
    let entries = entries::list(path)?;
    println!("*** Top {n} largest files ***");
    for entry in entries.iter().take(n) {
        println!("{entry}");
    }
    Ok(())
}
