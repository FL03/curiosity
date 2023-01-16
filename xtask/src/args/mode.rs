/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use clap::ValueEnum;


#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum Mode {
    #[default]
    Debug = 0,
    Release = 1
}

impl Mode {
    pub fn mode(&self) -> String {
        match self.clone() as i64 {
            0 => "debug".to_string(),
            _ => "release".to_string(),
        }
    }
}