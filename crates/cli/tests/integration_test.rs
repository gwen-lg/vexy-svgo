use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("vexy-svgo").unwrap();
    cmd.arg("--version").assert().success();
}
