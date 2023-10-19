use std::collections::HashMap;

struct Params<'a> {
    params: HashMap<&'a str, &'a str>
}

impl Params<'_> {}