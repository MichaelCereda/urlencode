extern crate clap;
extern crate urlencoding;
extern crate assert_cmd;

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
    cmd.assert().success().stdout(predicate::str::contains("URL encoding or decoding stdin"));
}
