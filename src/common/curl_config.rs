
pub fn get_http_method() -> Vec<& 'static str> {
    let options: Vec<&str> = vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"];
    options
}