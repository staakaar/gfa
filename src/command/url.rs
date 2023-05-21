
/// Url
/// enumに変更して列挙型に対してデータや振る舞いを持たせる
pub struct Url {
    scheme: String,
    sub_domain: String,
    domain: String,
    port: String,
    /// Endpoint Root
    root: String,
}

impl Url {
    fn new(self) -> Self {
        Self {
            scheme: self.scheme,
            sub_domain: self.sub_domain,
        }
    }

    /// schemeを入力するメソッド
    fn input_scheme(self) -> Self {}

    /// サブドメインを入力するメソッド

    /// ドメインを入力するメソッド

    /// ポート番号を入力するメソッド

    /// ルートを入力するメソッド

}