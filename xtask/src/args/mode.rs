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
    Release = 1,
}

impl Into<i64> for Mode {
    fn into(self) -> i64 {
        self as i64
    }
}

impl From<bool> for Mode {
    fn from(d: bool) -> Self {
        if d {
            Self::Release
        } else {
            Self::Debug
        }
    }
}

impl From<Mode> for bool {
    fn from(d: Mode) -> bool {
        match d as i64 {
            0 => false,
            1 => true,
            _ => false,
        }
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self.clone() as i64 {
            0 => "debug".to_string(),
            _ => "release".to_string(),
        }
    }
}
