/*
    Appellation: utils <modules>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Bundle;
use scsys::Result;
use std::path::{Path, PathBuf};
use std::{fs, io, process::Command};

/// A simple function wrapper for executing cargo related commands
pub fn cargo(args: Vec<&str>) -> Result<()> {
    command("cargo", args)
}
///
pub fn command(program: &str, args: Vec<&str>) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.current_dir(project_root());
    cmd.args(args.as_slice()).status()?;
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
pub fn dist_dir(target: Option<&str>) -> PathBuf {
    project_root().join(target.unwrap_or("dist"))
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
