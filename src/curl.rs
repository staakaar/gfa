use clap::Parser;

#[derive(Parser)]
struct Curl {
    url: &'static str,
    options: Vec<String>,
    query_params: Vec<HashMap<String, String>>,
    // ユーザーの入力によって型がさまざまあるためプログラム上でjsonへパースする
    json_param: String,
}

impl Curl {
    fn new() -> Self {
        Self {
            url,
            options,
            query_params,
        }
    }

    fn build_command() -> Result<()> {}
}