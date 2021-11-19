use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let mut arguments =std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let mut db = Database::from_disk()?;
    db.insert(key, value);
    db.flush()?;
    drop(db);
    Ok(())
}

struct Database {
    hashmap: HashMap<String, String>,
}

impl Database {
    fn from_disk() -> Result<Database, std::io::Error> {
        let mut hashmap = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.split("\t");
            let key = chunks.next().unwrap();
            let value = chunks.next().unwrap();
            hashmap.insert(key.to_string(), value.to_string());
        }
        Ok(Database { hashmap })
    }

    fn insert(&mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }

    fn flush(&self) -> std::io::Result<()> {
        let contents :String = todo!("implement this");
        std::fs::write("kv.db", contents)
    }
}