/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use wasm_bindgen::prelude::JsError;

/// Type alias for a [Result] of type T and error [JsError]
pub type JsResult<T = ()> = Result<T, JsError>;