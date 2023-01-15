/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use anyhow::Result;
use crate::cmds::Commands;
use clap::{ArgAction, Parser};


#[derive(Clone, Debug, Eq, Hash, Parser, PartialEq)]
#[clap(about, author, version)]
#[clap(arg_required_else_help(true))]
pub struct XtaskCLI {
    #[clap(subcommand)]
    pub cmd: Option<Commands>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub debug: bool
}

impl XtaskCLI {
    pub fn handle(&self) -> Result<&Self> {
        if let Some(cmd) = self.cmd.clone() {
            cmd.handle()?;
        }


        Ok(self)
    }
}

impl Default for XtaskCLI {
    fn default() -> Self {
        Self::parse()
    }
}