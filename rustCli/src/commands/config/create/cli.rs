use anyhow::Result;
use clap::{Args, Subcommand};
use crate::{app, platform};
use homedir::my_home;
use std::path::PathBuf;

/// Modify the config file
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let path = my_home().unwrap().unwrap().join(".rustcli").display().to_string();
        let canonicalized_path = platform::canonicalize_path(&path);
        debug!("Setting repository path to: {}", &path);


        let mut config = app::config().clone();
        config.repo = path;
        crate::config::save(config)?;
        return Ok(());
    }
}