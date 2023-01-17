/*
    Appellation: setup <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::command;
use clap::{ArgAction, Args};
use scsys::AsyncResult;

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Test {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub all: bool,
}

impl Test {
    pub async fn handle(&self) -> AsyncResult<&Self> {
        testing()?;
        Ok(self)
    }
}

///
pub fn testing() -> AsyncResult {
    command("cargo", &["test", "--all", "--all-features"])?;
    Ok(())
}
