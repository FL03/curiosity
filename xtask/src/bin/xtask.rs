use std::path::PathBuf;

/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use xtask_sdk as xtask;

use axum::{
    http::StatusCode,
    routing::get_service,
};
use clap::{Args, ArgAction, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use xtask::project_root;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let mut xtask = Xtask::new();
    xtask.process(Cli::parse()).await?;
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct Xtask;

impl Xtask {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn process(&mut self, cli: Cli) -> anyhow::Result<()> {
        if let Some(cmd) = cli.opts {
            match cmd {
                Opts::Build(b) => {
                    tracing::info!("Building...");
                    let mut args = vec!["build"];
                    if b.release {
                        args.push("--release");
                    }
                    if b.workspace {
                        args.push("--workspace");
                    }
                    xtask::cargo(args)?;
                }
                Opts::Serve(s) => {
                    tracing::info!("Serving the build artifacts...");
                    println!("{:?}", s);
                    wasm_server().await?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub opts: Option<Opts>,
    /// The directory to output the build artifacts to
    #[clap(long, short, default_value = "dist")]
    pub dist: String,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub verbose: bool
}


#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Subcommand)]
pub enum Opts {
    #[clap(about = "Build a target")]
    Build(Build),
    #[clap(about = "Serve the build artifacts")]
    Serve(Serve),
}

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Build {
    #[clap(long, short, default_value = "dist")]
    pub dist: String,
    #[clap(long, short)]
    pub package: String,
    #[clap(long, short, default_value = "wasm32-unknown-unknown")]
    pub platform: String,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool,
}

impl Build {
    pub fn build(&self) -> anyhow::Result<()> {
        let mut args = vec!["build"];
        if self.release {
            args.push("--release");
        }
        if self.workspace {
            args.push("--workspace");
        }

        Ok(())
    }
}

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Serve {
    /// The directory to serve the build artifacts from
    #[clap(long, short, default_value = "dist")]
    pub dist: String,
    /// The port to serve the build artifacts on
    #[clap(long, short, default_value = "8080")]
    pub port: u16
}

/// Quickstart a server for static assets
pub async fn wasm_server() -> anyhow::Result<()> {
    let root = project_root().to_str().unwrap().to_string();
    let dist = format!("{}/{}", root, "dist");

    let serve_dir = get_service(ServeDir::new(dist.as_str()));
    let app = axum::Router::new().nest_service("/", serve_dir);
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

pub async fn handle_io_error(err: std::io::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", err))
}

pub fn api(workdir: PathBuf) -> axum::Router {
    axum::Router::new().nest_service("/", get_service(ServeDir::new(workdir.as_os_str().to_str().unwrap())))
}

pub struct WasmServer {
    addr: std::net::SocketAddr,
    workdir: PathBuf
}

impl WasmServer {
    pub fn new(addr: std::net::SocketAddr, workdir: String) -> Self {
        Self { addr, workdir: project_root().join(workdir) }
    }
    pub async fn client(&self) -> axum::Router {
        axum::Router::new()
            .merge(api(self.workdir.clone()))
    }
    pub async fn serve(&self) -> anyhow::Result<()> {
        axum::Server::bind(&self.addr)
            .serve(self.client().await.into_make_service())
            .await
            .unwrap();
        Ok(())
    }
}

