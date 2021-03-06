use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
mod matcher;



#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("--path").arg("test/file/doesnt/exist");
    cmd.arg("--pattern").arg("blabla");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}