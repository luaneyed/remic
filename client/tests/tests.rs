use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

fn flush() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["flush"])
        .spawn()
        .unwrap();
}

// `remic` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("remic").unwrap().assert().failure();
}

// `remic -V` should print the version
#[test]
fn cli_version() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

// `remic get <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_get() {
    flush();
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["get", "key1"])
        .assert()
        .stdout(contains("Not Found"));
}

// `remic set <KEY> <VALUE>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_set() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["set", "key1", "value1"])
        .assert()
        .stdout(contains("Set Successfully"));
}

#[test]
fn cli_del() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["del", "key1"])
        .assert()
        .stdout(contains("Deleted Successfully"));
}

#[test]
fn cli_invalid_get() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["get"])
        .assert()
        .failure();

    Command::cargo_bin("remic")
        .unwrap()
        .args(&["get", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_set() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["set"])
        .assert()
        .failure();

    Command::cargo_bin("remic")
        .unwrap()
        .args(&["set", "missing_field"])
        .assert()
        .failure();

    Command::cargo_bin("remic")
        .unwrap()
        .args(&["set", "extra", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_rm() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["rm"])
        .assert()
        .failure();

    Command::cargo_bin("remic")
        .unwrap()
        .args(&["rm", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin("remic")
        .unwrap()
        .args(&["unknown", "subcommand"])
        .assert()
        .failure();
}
