/*
    Appellation: cmds <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

pub mod build;


use clap::{ArgAction, Parser};

#[derive(Clone, Debug, Default, Eq, Hash, Parser, PartialEq)]

pub struct CommandLineInterface {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub debug: bool,
}
