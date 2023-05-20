use clap::Subcommand;
use eyre::Result;

mod curl;

// @see Subcommand trait
#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum ICurl {
    Start(curl::Curl)
}

impl ICurl {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Start(curl) => {
                curl.interactive();
                Ok(())
            }
        }
    }
}