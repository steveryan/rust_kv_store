fn main() {
    let mut arguments =std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let write_result =  write_database(key, value);
    match write_result {
        Ok(_) => println!("write success"),
        Err(e) => println!("write error: {}", e),
    }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    std::fs::write("kv.db", format!("{},{}", key, value))
}
