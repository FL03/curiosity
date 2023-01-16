/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{cmds::wasm::WasmTarget, command};
use anyhow::Result;
use clap::{ArgAction, Args};
use std::sync::mpsc;

pub fn build_channels() -> (mpsc::Sender<Build>, mpsc::Receiver<Build>) {
    mpsc::channel()
}

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Build {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(value_enum)]
    pub target: Option<WasmTarget>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool,
}

impl Build {
    pub fn handle(&self) -> Result<&Self> {
        build_wasm(self.release.clone(), self.target.unwrap_or_default().clone(), self.workspace.clone())?;
        Ok(self)
    }
}

fn build_wasm(release: bool, target: WasmTarget, workspace: bool) -> Result<()> {
    let target = target.target();
    let mut args = vec!["build", "--target", target.as_str()];
    if release {
        tracing::info!("Building contents for release...");
        args.push("--release");
    }
    if workspace {
        tracing::info!("Building the workspace...");
        args.push("--workspace");
    }
    command("cargo", args.as_slice())?;

    Ok(())
}
