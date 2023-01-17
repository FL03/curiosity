/*
    Appellation: target <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum Target {
    WasmUnknownUnknown = 0,
    #[default]
    WasmWasi = 1,
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self.clone() as i64 {
            0 => "wasm32-unknown-unknown".to_string(),
            _ => "wasm32-wasi".to_string(),
        }
    }
}
