/// use

use clap::Parser;
use command::InCurl;
use eyre::Result;

/// mod
mod command;
mod common;

#[derive(Parser)]
struct ICurl {
    #[command(subcommand)]
    curl: InCurl,
}

impl ICurl {
    fn run(self) -> Result<()> {
        self.curl.run()
    }
}

fn main() -> Result<()>  {
    // icurlコマンド実行
    ICurl::parse().run()
}
