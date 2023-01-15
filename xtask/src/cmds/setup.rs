/*
    Appellation: setup <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::{command, dist_dir};
use anyhow::Result;
use clap::{ArgAction, Args};

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Setup {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub extras: bool,
}

impl Setup {
    pub fn handle(&self) -> Result<&Self> {
        setup(self.extras.clone())?;
        Ok(self)
    }
}

pub fn setup_artifacts() -> Result<()> {
    if std::fs::create_dir_all(&dist_dir()).is_err() {
        tracing::info!("Clearing out the previous build");
        std::fs::remove_dir_all(&dist_dir())?;
        std::fs::create_dir_all(&dist_dir())?;
    };
    Ok(())
}

pub fn setup(extras: bool) -> Result<()> {
    // Artifacts
    setup_artifacts()?;
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
    command("curl", &["-sSf", "https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh", "|", "bash"])?;
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
    };
    Ok(())
}
