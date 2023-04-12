/*
    Appellation: cmds <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

pub mod build;
pub mod setup;
pub mod start;
pub mod test;

use clap::Subcommand;
use scsys::AsyncResult;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Subcommand)]

pub enum Commands {
    Build(build::Build),
    Setup(setup::Setup),
    Start(start::WasmRunner),
    Test(test::Test),
}

impl Commands {
    pub async fn handle(&self) -> AsyncResult<&Self> {
        match self.clone() {
            Self::Build(build) => {
                build.handle().await?;
            }
            Self::Setup(setup) => {
                setup.handle().await?;
            }
            Self::Start(start) => {
                start.handle().await?;
            }
            Self::Test(test) => {
                test.handle().await?;
            }
        }
        Ok(self)
    }
}
