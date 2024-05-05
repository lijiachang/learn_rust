use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn main() {
    let shared_map = Arc::new(Mutex::new(HashMap::new()));

    let writer = {
        let shared_map = Arc::clone(&shared_map);
        thread::spawn(move || {
            let mut map = shared_map.lock().unwrap();
            map.insert("key1", "value1");
            map.insert("key2", "value2");
        })
    };

    let reader = {
        let shared_map = Arc::clone(&shared_map);
        thread::spawn(move || {
            let map = shared_map.lock().unwrap();
            for (key, value) in map.iter() {
                println!("{}: {}", key, value);
            }
        })
    };

    writer.join().unwrap();
    reader.join().unwrap();
}