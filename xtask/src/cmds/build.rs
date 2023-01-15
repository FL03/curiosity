/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::command;
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
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool,
}

impl Build {
    pub fn handle(&self) -> Result<&Self> {
        let mut args = vec!["build", "--target", "wasm32-wasi"];
        if self.release {
            tracing::info!("Building contents for release...");
            args.push("--release");
        }
        if self.workspace {
            tracing::info!("Building the workspace...");
            args.push("--workspace");
        }
        command("cargo", args.as_slice())?;
        Ok(self)
    }
}
