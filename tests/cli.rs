use assert_cmd::Command;

#[test]
fn binary_with_help_flag_prints_usage() {
    Command::cargo_bin("dstats")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage: dstats"));
}

#[test]
fn binary_with_no_args_prints_top_5_largest_files_under_working_directory() {
    Command::cargo_bin("dstats")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::str::contains("*** Top 5 largest files ***"));
}

#[test]
fn binary_with_path_arg_prints_the_top_5_largest_files_under_the_given_path() {
    let want = [
        "*** Top 5 largest files ***",
        "     7 B: ./testdata/en/world.txt",
        "     6 B: ./testdata/es/mundo.txt",
        "     6 B: ./testdata/en/hello.txt",
        "     5 B: ./testdata/es/hola.txt",
    ];
    Command::cargo_bin("dstats")
        .unwrap()
        .arg("./testdata")
        .assert()
        .success()
        .stdout(predicates::str::contains(want.join("\n")));
}

#[test]
fn binary_with_the_number_arg_prints_the_top_n_largest_files_under_the_current_working_directory() {
    Command::cargo_bin("dstats")
        .unwrap()
        .args(["-n", "10"])
        .assert()
        .success()
        .stdout(predicates::str::contains("*** Top 10 largest files ***"));
}
