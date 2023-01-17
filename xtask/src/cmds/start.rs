/*
    Appellation: start <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::{
    args::{Mode, Target},
    command,
    dist_dir
};
use clap::{ArgAction, Args};
use scsys::AsyncResult;
use tokio::sync::broadcast;

#[derive(Args, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WasmRunner {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(value_enum)]
    pub target: Option<Target>,
}

impl WasmRunner {
    pub async fn process(&self) -> AsyncResult<&Self> {
        let (_, mut rx) = broadcast::channel::<Mode>(1);

        let _mode = rx.recv().await?;

        Ok(self)
    }
    pub async fn handle(&self) -> AsyncResult<&Self> {
        tracing::info!("Initializing the wasm run system...");

        let target = self.target.unwrap_or_default().clone();
        
        run_wasm("curiosity")?;
        
        Ok(self)
    }
}

///
fn run_wasm(pkg: &str) -> AsyncResult {
    let path_dist = dist_dir(None).join(format!("{}.aot.wasm", pkg));
    let artifacts = path_dist.to_str().unwrap();

    tracing::info!("wasmedge: Running the application (aot)");
    command("wasmedge", &[artifacts])?;
    Ok(())
}
