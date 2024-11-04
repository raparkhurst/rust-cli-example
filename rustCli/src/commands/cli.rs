use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Rusty example app
#[derive(Parser, Debug)]
#[command(about = "rustCli:  Connecting you to my app", version, bin_name = "rustCli", disable_help_subcommand = true)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    // #[arg(short, long, default_value = "~/.rustcli/config.json")]
    // config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    //Config(super::config::Cli),
    Config(super::config::cli::Cli),
    Example(super::example::Cli),
    Cloudcmd(super::cloudcmd::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        debug!("verbosity set to:  {:?}", self.verbose);
        //debug!("config file: {:?}", self.config);



        match &self.command {
            //Commands::Config(cli) => cli.exec(),
            Commands::Config(cli) => cli.exec(),
            Commands::Example(cli) => cli.exec(),
            Commands::Cloudcmd(cli) => cli.exec(),
        }
    }
}
