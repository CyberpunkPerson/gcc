use std::io::Write;
use tempfile::NamedTempFile;
use std::process::Command;
use predicates::prelude::*;
use assert_cmd::prelude::*; 


#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("--path").arg(file.path());
    cmd.arg("--pattern").arg("test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Matched line number 1\nMatched line number 4"));

    Ok(())
}