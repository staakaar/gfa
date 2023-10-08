use clap::Subcommand;
use eyre::Result;

mod curl;

// @see Subcommand trait
#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum InCurl {
    Start(curl::CurlCmd)
}

impl InCurl {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Start(curl) => {
                let _ = curl.interactive();
                Ok(())
            }
        }
    }
}