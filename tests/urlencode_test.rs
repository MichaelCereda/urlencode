extern crate assert_cmd;
extern crate clap;
extern crate urlencoding;

use predicates::prelude::*;

#[test]
fn test_encode() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.write_stdin("Hello World!").unwrap();
    cmd.assert().success().stdout("Hello%20World%21\n");
}

#[test]
fn test_decode() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.arg("-d").write_stdin("Hello%20World%21").unwrap();
    cmd.assert().success().stdout("Hello World!\n");
}

#[test]
fn test_help() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("URL encoding or decoding stdin"));
}

#[test]
fn test_text_to_parse() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.arg("Hello World!");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello%20World%21\n"));
}

// Ignore standard input if text to parse is provided
#[test]
fn test_ignore_stdin_if_text_to_parse() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.arg("Hello World!")
        .write_stdin("I WILL BE IGNORED")
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello%20World%21\n"));
}
#[test]
fn test_ignore_stdin_if_text_to_parse_when_decoding() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.arg("Hello%20World%21")
        .arg("-d")
        .write_stdin("I WILL BE IGNORED")
        .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello World!\n"));
}

// don't print a new line if the nonewline flag is set
#[test]
fn test_nonewline() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.arg("-n").write_stdin("Hello World!").unwrap();
    cmd.assert().success().stdout("Hello%20World%21");
}

// don't print a new line if the nonewline flag is set even when decoding
#[test]
fn test_nonewline_when_decoding() {
    let mut cmd = assert_cmd::Command::cargo_bin("urlencode").unwrap();
    cmd.arg("-d")
        .arg("-n")
        .write_stdin("Hello%20World%21")
        .unwrap();
    cmd.assert().success().stdout("Hello World!");
}
