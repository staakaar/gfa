use std::collections::HashMap;

#[derive(Debug)]
pub struct Kvs {
    pub record: HashMap<String, String>
}

impl Kvs {
    pub fn new() -> Kvs {
        Kvs { record: HashMap::new() }
    }

    pub fn set(mut self, input_list: Vec<&str>) -> Kvs {
        for i in 0..input_list.len() / 2 {

            let key: String = input_list[i * 2].to_string();
            let value = input_list[i * 2 + 1].to_string();

            self.record.insert(key, value);
        }

        return self;
    }
}