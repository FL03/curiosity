/*
    Appellation: artifacts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::dist_dir;
use std::path::PathBuf;

pub struct Artifacts {
    pub workdir: PathBuf,
}

impl Artifacts {
    pub fn new(workdir: Option<&str>) -> Self {
        Self {
            workdir: dist_dir(workdir),
        }
    }
}
