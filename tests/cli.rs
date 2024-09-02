use assert_cmd::Command;

fn normalize_path_separators(output: &str) -> String {
    if cfg!(windows) {
        output.replace('/', "\\")
    } else {
        output.to_string()
    }
}

#[test]
fn binary_with_version_flag_prints_version() {
    Command::cargo_bin("spacehog")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn binary_with_help_flag_prints_description() {
    Command::cargo_bin("spacehog")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains(env!("CARGO_PKG_DESCRIPTION")));
}

#[test]
fn binary_with_help_flag_prints_usage() {
    Command::cargo_bin("spacehog")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage: spacehog"));
}

#[test]
fn binary_with_no_args_prints_top_5_largest_files_under_working_directory() {
    Command::cargo_bin("spacehog")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::str::contains("*** Top 5 largest files ***"));
}

#[test]
fn binary_with_path_arg_prints_the_top_5_largest_files_under_the_given_path() {
    let want = [
        r"\*\*\* Top 5 largest files \*\*\*",
        r"\d+ B ./testdata/en/world.txt",
        r"\d+ B ./testdata/es/mundo.txt",
        r"\d+ B ./testdata/en/hello.txt",
        r"\d+ B ./testdata/es/hola.txt",
    ];
    let want_normalized = normalize_path_separators(&want.join("\n"));
    Command::cargo_bin("spacehog")
        .unwrap()
        .arg("./testdata")
        .assert()
        .success()
        .stdout(predicates::str::is_match(want_normalized).unwrap());
}

#[test]
fn binary_with_the_number_arg_prints_the_top_n_largest_files_under_the_current_working_directory() {
    Command::cargo_bin("spacehog")
        .unwrap()
        .args(["-n", "10"])
        .assert()
        .success()
        .stdout(predicates::str::contains("*** Top 10 largest files ***"));
}

#[test]
fn binary_with_invalid_path_arg_prints_an_error_message_and_exits_with_failure_code() {
    #[cfg(windows)]
    let want = "The system cannot find the file specified";
    #[cfg(not(windows))]
    let want = "No such file or directory";

    Command::cargo_bin("spacehog")
        .unwrap()
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicates::str::contains(want));
}
