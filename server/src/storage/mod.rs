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

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn del(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn flushall(&mut self) {
        self.map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get() {
        let mut store = Store::new();

        store.set("key1".to_string(), "value1".to_owned());
        store.set("key2".to_owned(), "value2".to_owned());

        assert_eq!(store.get("key1"), Some(&"value1".to_owned()));
        assert_eq!(store.get("key2"), Some(&"value2".to_owned()));
    }

    // Should overwrite existent value
    #[test]
    fn overwrite_set() {
        let mut store = Store::new();

        store.set("key1".to_owned(), "value1".to_owned());
        assert_eq!(store.get("key1"), Some(&"value1".to_owned()));

        store.set("key1".to_owned(), "value2".to_owned());
        assert_eq!(store.get("key1"), Some(&"value2".to_owned()));
    }

    // Should get `None` when getting a non-existent key
    #[test]
    fn get_non_existent_value() {
        let mut store = Store::new();

        store.set("key1".to_owned(), "value1".to_owned());
        assert_eq!(store.get("key2"), None);
    }

    // Should get `None` when getting a non-existent key
    #[test]
    fn delete() {
        let mut store = Store::new();

        store.set("key1".to_owned(), "value1".to_owned());
        store.set("key2".to_owned(), "value2".to_owned());
        store.del("key1");
        assert_eq!(store.get("key1"), None);
        assert_eq!(store.get("key2"), Some(&"value2".to_owned()));
    }

    #[test]
    fn flushall() {
        let mut store = Store::new();

        store.set("key1".to_owned(), "value1".to_owned());
        store.flushall();
        assert_eq!(store.get("key1"), None);
    }
}
