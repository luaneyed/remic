use std::collections::HashMap;

pub struct Store {
    map: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}
