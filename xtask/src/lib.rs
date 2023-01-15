/*
    Appellation: xtask <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

pub use self::{interface::*, utils::*};

pub(crate) mod interface;
pub(crate) mod utils;

pub mod cmds;

///
pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;

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
                cmd.current_dir(scsys_xtask::project_root());
                let mut tmp = Vec::new();
                $(
                    tmp.push($y);
                )*
                cmd.args(tmp.as_slice()).status().expect("");
            )*
        }
    };
}
