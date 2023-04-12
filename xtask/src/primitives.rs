/*
    Appellation: primitives <modules>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{constants::*, statics::*, types::*};

mod constants {
    /// The default host for the server.
    pub const LOCALHOST: &str = "0.0.0.0";
    /// The default port for the server.
    pub const DEFAULT_PORT: u16 = 8080;
}

mod statics {}

mod types {
    use std::collections::HashMap;

    /// A bundle of commands to be executed.
    pub type Bundle<T = String> = HashMap<T, Vec<Vec<T>>>;
}