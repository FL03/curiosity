/*
    Appellation: xtask <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{primitives::*, utils::*};


mod primitives;
mod utils;

pub mod commands;
pub mod pipes;
pub mod wasm;


///
#[macro_export]
macro_rules! cmd {
    ($(
        $x:expr;
        [ $( $y:expr ),* ]
    );*) => {
        {
            $(
                let mut cmd = std::process::Command::new($x);
                cmd.current_dir(xtask_sdk::project_root());
                let mut tmp = Vec::new();
                $(
                    tmp.push($y);
                )*
                cmd.args(tmp.as_slice()).status().expect("");
            )*
        }
    };
    ($(
        $x:expr;
        $y:expr;
        [ $( $z:expr ),* ]
    );*) => {
        {
            $(
                let mut cmd = std::process::Command::new($x);
                cmd.current_dir(xtask_sdk::project_root());
                let mut tmp = vec![$y];
                $(
                    tmp.push($z);
                )*
                cmd.args(tmp.as_slice()).status().expect("");
            )*
        }
    };
}
