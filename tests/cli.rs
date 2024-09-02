use assert_cmd::Command;

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
    #[cfg(windows)]
    let want = [
        "*** Top 5 largest files ***",
        "8 B ./testdata\\en\\world.txt",
        "7 B ./testdata\\es\\mundo.txt",
        "7 B ./testdata\\en\\hello.txt",
        "6 B ./testdata\\es\\hola.txt",
    ];
    #[cfg(not(windows))]
    let want = [
        "*** Top 5 largest files ***",
        "7 B ./testdata/en/world.txt",
        "6 B ./testdata/es/mundo.txt",
        "6 B ./testdata/en/hello.txt",
        "5 B ./testdata/es/hola.txt",
    ];
    Command::cargo_bin("spacehog")
        .unwrap()
        .arg("./testdata")
        .assert()
        .success()
        .stdout(predicates::str::contains(want.join("\n")));
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
    let want = "The system cannot find the path specified";
    #[cfg(not(windows))]
    let want = "No such file or directory";
    Command::cargo_bin("spacehog")
        .unwrap()
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicates::str::contains(want));
}
