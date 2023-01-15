/*
    Appellation: setup <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::command;
use anyhow::Result;
use clap::{ArgAction, Args};

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Test {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub all: bool,
}

///
pub fn testing() -> Result<()> {
    command("cargo", &["test", "--all", "--all-features"])
}
