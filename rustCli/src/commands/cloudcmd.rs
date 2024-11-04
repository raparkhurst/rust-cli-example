use anyhow::Result;
use clap::Args;
use std::process::{exit, Command};
use crate::app;



#[derive(Args, Debug)]
#[command(about = "Example cloudcmd template related sub commands", version)]
pub struct Cli {
    #[arg(short, long)]
    text: String,

}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        println!("cloudcmd subcommand");

        trace!("trace {}", self.text);
        debug!("debug {}", self.text);
        info!("info {}", self.text);
        success!("success {}", self.text);
        waiting!("waiting {}", self.text);
        warn!("warn {}", self.text);
        error!("error {}", self.text);
        display!("display {}", self.text);
        critical!("critical {}", self.text);

        debug!("{:?}", app::get_settings());

        Ok(())
    }
}
