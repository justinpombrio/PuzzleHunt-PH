extern crate ph;

use std::io::{self, Write};
use ph::database::Database;

fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn main() {
    let answer = prompt("This will DELETE the WHOLE database. Are you sure? (yes/no) ");
    if answer.trim() == "yes" {
        let db = Database::new();
        println!("Wiping database and re-initializing with test data.");
        db.clear();
        db.init_test();
        println!("Done");
    } else {
        println!("Cancelled");
    }
}
