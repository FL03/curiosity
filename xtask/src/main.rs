/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use xtask_sdk::*;

use axum::{http::StatusCode, response::IntoResponse, routing::get_service};
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");

    handler().await?;

    Ok(())
}

pub fn base_matches(sc: Command) -> ArgMatches {
    command!()
        .arg(
            arg!(
                config: -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(release:
                -r --release ... "Flag the system to run in release"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(port: -p --port <PORT>)
                .help("Network port to use")
                .value_parser(value_parser!(u16).range(1..))
                .default_value("8080"),
        )
        .arg(arg!(up: -u --up ...).action(ArgAction::SetTrue))
        .arg_required_else_help(true)
        .propagate_version(true)
        .subcommand(sc)
        .subcommand_required(false)
        .get_matches()
}

pub fn cli() -> ArgMatches {
    base_matches(
        Command::new("app")
            .about("Application commands")
            .arg(arg!(build: -b --build <BUILD> "Build the workspace"))
            .arg(arg!(serve: -s --serve <PKG> "Build the workspace")),
    )
}

pub async fn handler() -> anyhow::Result<()> {
    let root = project_root().to_str().unwrap().to_string();
    let dist = format!("{}/{}", root, "dist");

    let matches = cli();

    let release: bool = matches.get_one::<bool>("release").unwrap().clone();

    let port: u16 = *matches.get_one::<u16>("port").unwrap();

    if let Some(_up) = matches.clone().get_one::<bool>("up") {
        // wasm_server(dist.as_str(), Some(port)).await?;
        xtask_sdk::cmds::start::runner(release, "curiosity")?;
    }

    println!("{:?}", port);
    Ok(())
}

/// Quickstart a server for static assets
pub async fn wasm_server(path: &str, port: Option<u16>) -> anyhow::Result<()> {
    let serve_dir = get_service(ServeDir::new(path)).handle_error(handle_error);
    let app = axum::Router::new().nest_service("/", serve_dir);
    axum::Server::bind(&std::net::SocketAddr::from((
        [0, 0, 0, 0],
        port.unwrap_or(8080),
    )))
    .serve(app.into_make_service())
    .await
    .unwrap();
    Ok(())
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
