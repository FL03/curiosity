/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{interface::*, primitives::*, utils::*};

pub(crate) mod interface;
pub(crate) mod primitives;
pub(crate) mod utils;

pub mod args;
pub mod artifacts;
pub mod cmds;
pub mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> scsys::AsyncResult {
    tracing_subscriber::fmt::init();

    XtaskCLI::default().handle().await?;

    Ok(())
}
