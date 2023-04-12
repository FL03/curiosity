/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;

pub type AsyncResult<T = ()> = Result<T, AsyncError>;
