use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let mut arguments =std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    write_database(key, value)
}

struct Database {
    hashmap: HashMap<String, String>,
}

impl Database {
    fn from_disk() -> Database {
        db = Database {
            hashmap: HashMap::new(),
        }
    }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    std::fs::write("kv.db", format!("{}\t{}", key, value))
}
