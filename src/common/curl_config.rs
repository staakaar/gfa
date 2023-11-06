// TODO モジュールかしたい

pub fn get_http_method() -> Vec<& 'static str> {
    let options: Vec<&str> = vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"];
    options
}

pub fn get_protocol() -> Vec<& 'static str> {
    let options: Vec<&str> = vec!["HTTP", "FILE"];
    options
}

pub fn get_host() -> Vec<& 'static str> {
    let options: Vec<&str> = vec!["LocalHost", "Other"];
    options
}

pub fn get_authorization() -> Vec<& 'static str> {
    let options: Vec<&str> = vec!["Enter manually", "Read environment variables"];
    options
}

pub fn is_params() -> Vec<& 'static str> {
    let options: Vec<&str> = vec!["YES", "NO"];
    options
}

pub fn is_header() -> Vec<& 'static str> {
    let options: Vec<&str> = vec!["YES", "NO"];
    options
}
