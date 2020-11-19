use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RudisDb {
    pub db: Arc<Mutex<HashMap<String, i64>>>,
}

impl RudisDb {
    pub fn new() -> RudisDb {
        let map: HashMap<String, i64> = HashMap::new();
        RudisDb {
            db: Arc::new(Mutex::new(map)),
        }
    }

    pub fn put(self, command: String) {
        let mut map = self.db.lock().unwrap();
        map.insert("K".to_string(), 42);
        println!("Put command: {}", command);
    }

    pub fn get(self, _command: String) {
        let map = self.db.lock().unwrap();
        println!("Get command: {}", map.get("K").unwrap());
    }
}
