use clap::Subcommand;
use eyre::Result;

mod interactive;
mod protocol;
mod http;
mod params;
mod headers;
mod options;
mod authorization;
mod curl_input;
mod request;
mod request_body;



// @see Subcommand trait
#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum InCurl {
    #[command(subcommand)]
    Start(interactive::Cmd)
}

impl InCurl {

    #[tokio::main(flavor = "current_thread")]
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Start(interactive) => interactive.run().await,
        }

        Ok(())
    }
}