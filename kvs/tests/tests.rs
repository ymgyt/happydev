use assert_cmd::prelude::*;
use kvs::Kvs;
use predicates::str::contains;
use std::process::Command;

#[test]
fn cli_no_args() {
    Command::cargo_bin("kvs").unwrap().assert().failure();
}

#[test]
fn cli_version() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_put_get_delete() -> Result<(), anyhow::Error> {
    let tmp_dir = tempdir::TempDir::new("")?;
    let tmp_path = tmp_dir.path().join("test.kvs");
    let mut kvs = Kvs::new(tmp_path)?;

    kvs.put("key1", "value1X".to_owned())?;
    assert_eq!(kvs.get::<String>("key1")?, "value1X".to_owned());

    kvs.put("key1", "value1Y".to_owned())?;
    assert_eq!(kvs.get::<String>("key1")?, "value1Y".to_owned());

    assert!(kvs.get::<String>("not-exist").unwrap_err().is_not_found());

    assert_eq!(kvs.delete::<String>("key1")?, Some("value1Y".to_owned()));
    assert_eq!(kvs.delete::<String>("key1")?, None::<String>);

    Ok(())
}
