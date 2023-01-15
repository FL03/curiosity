/*
    Appellation: xtask <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{interface::*, utils::*};

pub(crate) mod interface;
pub(crate) mod utils;

pub mod cmds;
pub mod server;

///
pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;
