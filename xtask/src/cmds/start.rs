/*
    Appellation: start <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::command;
use anyhow::Result;
use clap::{ArgAction, Args};

///
pub fn runner(release: bool, pkg: &str) -> Result<()> {
    let mut target = "target/wasm32-wasi".to_string();
    if release {
        target = format!("{}/{}", target, "release");
    } else {
        target = format!("{}/{}", target, "debug");
    }
    target = format!("{}/{}.wasm", target, pkg);
    command("wasmedge", &[target.as_str()])
}

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Start {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool,
}

impl Start {
    pub fn handle(&self) -> Result<&Self> {
        runner(self.release.clone(), "curiosity")?;
        Ok(self)
    }
}

