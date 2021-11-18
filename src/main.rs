fn main() -> Result<(), std::io::Error> {
    let mut arguments =std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    write_database(key, value)
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
    std::fs::write("kv.db", format!("{}\t{}", key, value))
}
