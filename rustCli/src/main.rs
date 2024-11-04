#[macro_use]
mod macros;
mod app;
mod commands;
mod modules;
mod config;
mod platform;


use std::collections::HashMap;
use anyhow::Result;
use clap::Parser;

use crate::commands::cli::Cli;


fn main() -> Result<()> {
    let cli = Cli::parse();
    app::set_global_verbosity(cli.verbose.log_level_filter());
    app::set_global_config(config::load()?);


    // TODO:  Fix this so that if the config hasn't been created yet it creates it without just erroring out.
    //app::set_global_settings(config::settings());

    cli.exec()
}