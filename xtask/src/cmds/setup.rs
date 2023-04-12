/*
    Appellation: setup <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::{artifacts, command};
use clap::{ArgAction, Args};
use scsys::AsyncResult;

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Setup {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub extras: bool,
}

impl Setup {
    pub async fn handle(&self) -> AsyncResult<&Self> {
        // Artifacts
        artifacts(None)?;
        setup_wasm()?;
        setup_wasmedge()?;
        Ok(self)
    }
}

pub fn setup_wasm() -> AsyncResult {
    tracing::info!("Setting up the workspace for WebAssembly...");
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
    Ok(())
}

fn setup_wasmedge() -> AsyncResult {
    tracing::info!("Installing wasmedge...");
    command("sh", &["scripts/wasmedge.sh"])?;
    command("source", &["$HOME/.wasmedge/env"])?;
    Ok(())
}
