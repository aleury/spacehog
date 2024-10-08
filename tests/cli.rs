use std::io::Write;
use std::{fs::File, io::BufRead};

use assert_cmd::Command;
use tempfile::{tempdir, tempdir_in, TempDir};

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
fn binary_with_path_arg_prints_the_top_n_largest_files_under_the_given_path() {
    let output = Command::cargo_bin("spacehog")
        .unwrap()
        .arg("./testdata")
        .assert()
        .success()
        .get_output()
        .clone();
    let lines: Vec<String> = output.stdout.lines().map_while(Result::ok).collect();
    assert!(lines[0].contains("*** Top 4 largest files ***"));
    assert!(lines[1].ends_with("world.txt"));
    assert!(lines[2].ends_with("mundo.txt"));
    assert!(lines[3].ends_with("hello.txt"));
    assert!(lines[4].ends_with("hola.txt"));
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
fn binary_reports_that_the_directory_is_empty_if_it_contains_zero_files() {
    let dir = TempDir::new().expect("failed to create temporary directory");
    Command::cargo_bin("spacehog")
        .unwrap()
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("No files found."));
}

#[test]
fn binary_ignores_hidden_files_and_directories_by_default() {
    use std::io::Write;
    let parent_dir = tempdir().expect("failed to create temporary directory");
    let hidden_dir = tempdir_in(parent_dir.path()).expect("failed to create temporary directory");
    let temp_file_path = hidden_dir.path().join("test.txt");
    let mut temp_file = File::create(temp_file_path).expect("failed to create temporary file");
    write!(temp_file, "hello test").unwrap();

    Command::cargo_bin("spacehog")
        .unwrap()
        .arg(parent_dir.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("No files found."));
}

#[test]
fn binary_includes_hidden_files_and_directories_when_given_hidden_flag() {
    let parent_dir = tempdir().expect("failed to create temporary directory");
    let hidden_dir = tempdir_in(parent_dir.path()).expect("failed to create temporary directory");
    let temp_file_path = hidden_dir.path().join("test.txt");
    let mut temp_file = File::create(temp_file_path).expect("failed to create temporary file");
    write!(temp_file, "hello test").unwrap();

    Command::cargo_bin("spacehog")
        .unwrap()
        .arg(parent_dir.path())
        .arg("--hidden")
        .assert()
        .success()
        .stdout(predicates::str::contains("*** Top 1 largest files ***"));
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
