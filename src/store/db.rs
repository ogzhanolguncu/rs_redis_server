    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    #[derive(Debug, Clone)]
    pub struct Cache {
        data: Arc<RwLock<HashMap<String, String>>>,
    }

    impl Cache {
        pub fn new() -> Self {
            Self {
                data: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        pub fn set(&self, key: String, value: String) {
            let mut data = self.data.write().unwrap();
            data.insert(key, value);
        }

        pub fn get(&self, key: &str) -> Option<String> {
            let data = self.data.read().unwrap();
            data.get(key).cloned()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_initialize_and_get_set() {
            let cache = Cache::new();
            cache.set(String::from("name"), String::from("The Wizard of Oz"));
            assert_eq!("The Wizard of Oz", cache.get("name").unwrap());
        }
    }
