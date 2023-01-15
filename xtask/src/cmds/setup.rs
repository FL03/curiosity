/*
    Appellation: setup <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::{command, dist_dir};
use anyhow::Result;
use clap::{Args, ArgAction};

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Setup {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool
}


pub fn setup(extras: bool) -> Result<()> {
    // Artifacts
    if std::fs::create_dir_all(&dist_dir()).is_err() {
        tracing::info!("Clearing out the previous build");
        std::fs::remove_dir_all(&dist_dir())?;
        std::fs::create_dir_all(&dist_dir())?;
    };
    command("nix", &["flake", "update"])?;
    command("rustup", &["default", "nightly"])?;
    command(
        "rustup",
        &[
            "target",
            "add",
            "wasm32-unknown-unknown",
            "wasm32-wasi",
            "--toolchain",
            "nightly",
        ],
    )?;
    if extras {
        command(
            "rustup",
            &[
                "component",
                "add",
                "clippy",
                "rustfmt",
                "--toolchain",
                "nightly",
            ],
        )?;
        command("npm", &["install", "-g", "wasm-pack"])?;
    };
    Ok(())
}
