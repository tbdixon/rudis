use std::sync::{Mutex, Arc};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RudisDb {
    db: Arc<Mutex<HashMap<String, i32>>>
}

impl RudisDb {
    pub fn new() -> RudisDb {
        let map: HashMap<String, i32> = HashMap::new();
        RudisDb{ db: Arc::new(Mutex::new(map)) }
    }

    pub fn put(self, command: String){
        let mut map = self.db.lock().unwrap();
        map.insert("K".to_string(), 42);
        println!("Put command: {}", command);
    }

    pub fn get(self, _command: String){
        let map = self.db.lock().unwrap();
        println!("Get command: {}", map.get("K").unwrap());
    }

}
