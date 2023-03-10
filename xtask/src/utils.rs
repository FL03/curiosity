/*
    Appellation: utils <modules>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Bundle;
use scsys::Result;
use std::path::{Path, PathBuf};
use std::{fs, io, process::Command};

/// This function setups the artifacts directory
pub fn artifacts(path: Option<&str>) -> Result {
    tracing::info!("setting up the artifacts directory");
    let dist = dist_dir(path);
    if std::fs::create_dir_all(&dist).is_err() {
        tracing::info!("Clearing out the previous build");
        std::fs::remove_dir_all(&dist)?;
        std::fs::create_dir_all(&dist)?;
    };
    Ok(())
}
///
pub fn cargo(args: &[&str]) -> Result {
    command("cargo", args)
}
///
pub fn clippy() -> Result<()> {
    cargo(&["clippy", "--all", "--allow-dirty", "--fix"])
}
///
pub fn command(program: &str, args: &[&str]) -> Result {
    let mut cmd = Command::new(program);
    cmd.current_dir(project_root());
    cmd.args(args).status()?;
    Ok(())
}
///
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
///
pub fn dist_dir(path: Option<&str>) -> PathBuf {
    project_root().join(path.unwrap_or(".artifacts/dist"))
}
///
pub fn execute_bundle(bundle: Bundle<&str>) -> Result {
    for k in bundle.keys() {
        // Step 1: Rustup
        for i in 0..bundle[k].len() {
            let mut cmd = Command::new(k);
            cmd.current_dir(project_root());
            cmd.args(bundle[k][i].clone().as_slice()).status()?;
        }
    }
    Ok(())
}
///
pub fn nix(args: &[&str]) -> Result {
    command("nix", args)
}
/// Fetch the package name
pub fn package_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}
/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
///
pub fn rustfmt() -> Result {
    cargo(&["fmt", "--all"])
}
///
pub fn rustup(args: &[&str]) -> Result {
    command("rustup", args)
}
