/*
    Appellation: start <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::{args::Mode, command};
use anyhow::Result;
use clap::{ArgAction, Args, ValueEnum};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum WasmTarget {
    WasmUnknownUnknown = 0,
    #[default]
    WasmWasi = 1
}

impl WasmTarget {
    pub fn target(&self) -> String {
        match self.clone() as i64 {
            0 => "wasm32-unknown-unknown".to_string(),
            _ => "wasm32-wasi".to_string(),
        }
    }
}

#[derive(Args, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WasmRunner {
    #[arg(value_enum)]
    pub mode: Option<Mode>,
    #[arg(value_enum)]
    pub target: Option<WasmTarget>
}

impl WasmRunner {
    pub fn handle(&self) -> Result<&Self> {
        let mode = self.mode.unwrap_or_default().clone();
        let target = self.target.unwrap_or_default().clone();

        tracing::info!("Initializing the wasm run system...");
        run_wasm(target, mode, "curiosity")?;

        Ok(self)
    }
}

///
fn run_wasm(target: WasmTarget, mode: Mode, pkg: &str) -> Result<()> {
    let workdir = format!("target/{}/{}/{}.wasm", target.target(), mode.mode(), pkg.clone());
    let artifacts = format!(".artifacts/dist/{}.aot.wasm", pkg);

    tracing::info!("wasmedgec: Running the AOT compiler...");
    command("wasmedgec", &[workdir.as_str(), artifacts.clone().as_str()])?;
    tracing::info!("wasmtime: Running the application");
    command("wasmedge", &[artifacts.as_str()])?;
    Ok(())
}