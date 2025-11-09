mod database;
mod error;

use database::Database;

fn main() {
    let mut db = Database::new();
    db.set("foo".to_string(), "bar".to_string());
    let val = db.get("foo".to_string());
    match val {
        Ok(v) => println!("Value for 'foo': {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
