/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{
    args::{Mode, Target},
    command, dist_dir,
};
use clap::{ArgAction, Args};
use scsys::AsyncResult;
use tokio::sync::broadcast;

#[derive(Args, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Build {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(value_enum)]
    pub target: Option<Target>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool,
}

impl Build {
    pub async fn process(&self) -> AsyncResult {
        let (_, mut rx) = broadcast::channel::<Mode>(1);
        let _mode = rx.recv().await?;
        Ok(())
    }
    pub async fn handle(&self) -> AsyncResult {
        build_wasm(
            self.release,
            self.target.unwrap_or_default(),
            self.workspace,
        )?;

        Ok(())
    }
}

fn compile_wasmedge(target: Target, mode: Mode, pkg: &str) -> AsyncResult {
    let workdir = format!(
        "target/{}/{}/{}.wasm",
        target.to_string(),
        mode.to_string(),
        pkg.clone()
    );
    let path_wasm = dist_dir(None).join(format!("{pkg}.wasm"));
    let path_aot = dist_dir(None).join(format!("{pkg}.aot.wasm"));
    let (compiled, aot) = (path_wasm.to_str().unwrap(), path_aot.to_str().unwrap());

    command("cp", &[workdir.as_str(), compiled])?;
    tracing::info!("wasmedgec: Running the AOT compiler...");
    command("wasmedgec", &[workdir.as_str(), aot])?;

    Ok(())
}

fn build_wasm(release: bool, target: Target, workspace: bool) -> AsyncResult {
    let targ = target.clone().to_string();
    let mut args = vec!["build", "--target", targ.as_str()];
    if release {
        tracing::info!("Building contents for release...");
        args.push("--release");
    }
    if workspace {
        tracing::info!("Building the workspace...");
        args.push("--workspace");
    }
    command("cargo", args.as_slice())?;
    compile_wasmedge(target, Mode::from(release), "curiosity")?;
    Ok(())
}
