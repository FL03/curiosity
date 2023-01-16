/*
    Appellation: cmds <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

pub mod build;
pub mod setup;
pub mod start;
pub mod test;
pub mod wasm;

use anyhow::Result;
use clap::Subcommand;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Subcommand)]

pub enum Commands {
    Build(build::Build),
    Setup(setup::Setup),
    Start(wasm::WasmRunner),
    Test(test::Test),
}

impl Commands {
    pub fn handle(&self) -> Result<&Self> {
        match self.clone() {
            Self::Build(build) => {
                build.handle()?;
            },
            Self::Setup(setup) => {
                setup.handle()?;
            },
            Self::Start(start) => {
                start.handle()?;
            },
            Self::Test(test) => {
                test.handle()?;
            }
        }
        Ok(self)
    }
}
