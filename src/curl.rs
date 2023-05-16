use clap::Parser;

#[derive(Parser)]
struct Curl {
    url: &'static str,
    options: Vec<String>
}

impl Curl {
    fn new() -> Self {
        Self {
            url,
            options,
        }
    }

    fn build_command() -> Result<()> {}
}