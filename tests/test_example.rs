use anyhow::{ensure, Context, Result};
use std::env;
use std::path::Path;
use std::process::Command;
use std::sync::Once;

static SETUP: Once = Once::new();

fn run(dir: &str, f: impl FnOnce(&mut Command)) -> Result<()> {
  let root = env::temp_dir().join("rustc-plugin");

  SETUP.call_once(|| {
    let mut cmd = Command::new("cargo");
    cmd.args([
      "install",
      "--path",
      "examples/print-all-items",
      "--debug",
      "--locked",
      "--root",
    ]);
    cmd.arg(&root);
    let status = cmd.status().unwrap();
    if !status.success() {
      panic!("installing example failed")
    }
  });

  let mut cmd = Command::new("cargo");
  cmd.arg("print-all-items");

  let path = format!(
    "{}:{}",
    root.join("bin").display(),
    env::var("PATH").unwrap_or_else(|_| "".into())
  );
  cmd.env("PATH", path);

  let here = Path::new(file!());
  let ws = here.parent().unwrap().join(dir);
  cmd.current_dir(ws);

  f(&mut cmd);

  let status = cmd.status().context("Process failed")?;
  ensure!(status.success(), "Process exited with non-zero exit code");

  Ok(())
}

#[test]
fn basic() -> Result<()> {
  run("workspaces/basic", |_cmd| {})
}

#[test]
fn basic_with_arg() -> Result<()> {
  run("workspaces/basic", |cmd| {
    cmd.arg("-a");
  })
}

#[test]
fn multi() -> Result<()> {
  run("workspaces/multi", |_cmd| {})
}