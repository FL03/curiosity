/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum Platform {
    #[default]
    Linux = 0,
    MacOS = 1,
    Windows = 2,
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match *self {
            Self::Linux => "linux".to_string(),
            Self::MacOS => "apple".to_string(),
            Self::Windows => "windows".to_string(),
        }
    }
}
