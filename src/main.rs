use std::{collections::HashMap};

fn main() -> Result<(), std::io::Error> {
    let mut arguments =std::env::args().skip(1);
    let command = arguments.next().unwrap();
    let key = arguments.next();
    if key.is_none() {
        println!("No key provided");
        return Ok(());        
    }
    let key = key.unwrap();
    let mut db = Database::from_disk()?;
    let value = arguments.next();
    if command == "get" {
        db.retrieve(key)?;
    } else if command == "del" || command == "delete" {
        db.delete(key)?;
        db.flush()?;
        drop(db);
    } else if command == "set" && value.is_some() {
        let value = value.unwrap();
        db.insert(key, value);
        db.flush()?;
        drop(db);
    } else if command == "set" && value.is_none() {
        println!("Set commands must include a value argument");
    } else {
        println!("Unknown command: {}", command);
    }
    
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
            let mut chunks = line.split('\t');
            let key = chunks.next().unwrap();
            let value = chunks.next().unwrap();
            hashmap.insert(key.to_string(), value.to_string());
        }
        Ok(Database { hashmap })
    }

    fn insert(&mut self, key: String, value: String) {
        println!("Ok! I'll remember that {} is {}", key, value);
        self.hashmap.insert(key, value);
    }

    fn flush(&self) -> std::io::Result<()> {
        let mut contents: Vec<String> = Vec::new();
        for (key, value) in self.hashmap.iter() {
            let line = format!("{}\t{}", key, value);
            contents.push(line);
        }
        let contents = contents.join("\n");
        std::fs::write("kv.db", contents)
    }

    fn retrieve(&self, key: String) -> Result<(), std::io::Error> {
        match self.hashmap.get(&key) {
            Some(value) => println!("{}", value),
            None => println!("Key not found"),
        }
        Ok(())
    }

    fn delete(&mut self, key: String) -> Result<(), std::io::Error> {
        match self.hashmap.get(&key) {
            Some(value) => {
                println!("deleted {} {}", key, value);
                self.hashmap.remove(&key);
            },
            None => println!("Key not found"),
        }
        Ok(())
    }
}