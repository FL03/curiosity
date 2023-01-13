/*
    Appellation: curiosity <wasm>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use curiosity::app;

use dioxus_web::Config;

fn main() {
    dioxus_web::launch_with_props(app, (), Config::new().hydrate(true));
}
