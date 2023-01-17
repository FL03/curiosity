/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::{args::Mode, cmds::Commands};
use clap::{ArgAction, Parser};
use scsys::AsyncResult;
use tokio::sync::broadcast;

#[derive(Clone, Debug, Eq, Hash, Parser, PartialEq)]
#[clap(about, author, version)]
#[clap(propagate_version = true)]
#[clap(arg_required_else_help(true))]
pub struct XtaskCLI {
    #[clap(subcommand)]
    pub cmd: Option<Commands>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
}

impl XtaskCLI {
    pub async fn handle(&self) -> AsyncResult<&Self> {
        let (tx, _rx) = broadcast::channel::<Mode>(1);
        tx.send(Mode::from(self.release.clone()))?;
        handle_cli(self).await?;
        Ok(self)
    }
}

impl Default for XtaskCLI {
    fn default() -> Self {
        Self::parse()
    }
}

pub async fn handle_cli(cli: &XtaskCLI) -> AsyncResult<&XtaskCLI> {
    // Handle the Subcommands
    if let Some(cmd) = cli.cmd.clone() {
        cmd.handle().await?;
    }
    Ok(cli)
}
