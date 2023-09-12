use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Cache {
    data: Arc<RwLock<HashMap<String, String>>>,
    expirations: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Cache {
    pub fn new() -> Self {
        let data = Arc::new(RwLock::new(HashMap::new()));
        let expirations = Arc::new(RwLock::new(HashMap::new()));
        let cache = Self { data, expirations };

        let data_clone = cache.data.clone();
        let expirations_clone = cache.expirations.clone();
        thread::spawn(move || loop {
            let now = Instant::now();
            let keys_to_remove: Vec<String> = {
                let expirations = expirations_clone.read().unwrap();
                expirations
                    .iter()
                    .filter_map(|(key, &time)| if time <= now { Some(key.clone()) } else { None })
                    .collect()
            };

            if !keys_to_remove.is_empty() {
                let mut data = data_clone.write().unwrap();
                for key in keys_to_remove {
                    data.remove(&key);
                }
            }

            thread::sleep(Duration::from_secs(1));
        });

        cache
    }

    pub fn set_with_expiration(
        &self,
        key: String,
        value: String,
        secs: Duration,
    ) -> Result<(), &'static str> {
        if let Ok(mut data) = self.data.write() {
            if let Ok(mut expirations) = self.expirations.write() {
                data.insert(key.clone(), value);
                expirations.insert(key, Instant::now() + secs);
                Ok(())
            } else {
                Err("Could not acquire expirations write lock")
            }
        } else {
            Err("Could not acquire data write lock")
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<(), &'static str> {
        if let Ok(mut data) = self.data.write() {
            data.insert(key, value);
            Ok(())
        } else {
            Err("Could not acquire data write lock")
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, &'static str> {
        match self.data.read() {
            Ok(data) => Ok(data.get(key).cloned()),
            Err(_) => Err("Could not acquire data read lock"),
        }
    }

    pub fn exists(&self, key: &str) -> Result<bool, &'static str> {
        match self.data.read() {
            Ok(data) => Ok(data.contains_key(key)),
            Err(_) => Err("Could not acquire data read lock"),
        }
    }

    pub fn del(&self, key: &str) -> Result<Option<String>, &'static str> {
        if let Ok(mut data) = self.data.write() {
            Ok(data.remove(key))
        } else {
            Err("Could not acquire data write lock")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize_and_get_set() {
        let cache = Cache::new();
        cache.set(String::from("name"), String::from("The Wizard of Oz")).unwrap();
        assert_eq!("The Wizard of Oz", cache.get("name").unwrap().unwrap());
    }

    #[test]
    fn should_set_and_get_expiration() {
        let cache = Cache::new();
        cache.set_with_expiration(
            String::from("name"),
            String::from("The Wizard of Oz"),
            Duration::from_secs(3),
        ).unwrap();
        thread::sleep(Duration::from_secs(4));
        assert!(cache.get("name").unwrap().is_none());
    }
}
