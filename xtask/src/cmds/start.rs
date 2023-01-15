/*
    Appellation: start <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::command;
use anyhow::Result;
use clap::{Args, ArgAction};

///
pub fn runner(release: bool) -> Result<()> {
    let mut args = vec!["run"];
    if release {
        args.push("--release");
    }
    args.push("--");
    args.push("--h");
    command("cargo", args.as_slice())
}

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Start {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool
}

