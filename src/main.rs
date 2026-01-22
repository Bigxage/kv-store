use std::collections::HashMap;
use std::io;

struct Database {
    //todo: this struct needs a field named 'map' that stores key (string) and value (string)
    //hint: look at chapter 8 of the rust book -> HashMaps
    map: HashMap<String, String>,
}

impl Database {
    //constructor: creates a new empty database
    fn new() -> Database {
        //todo: return a new database with an empty hashmap
        Database {
            map: HashMap::new(),
        }
    }

    //insert a value
    fn insert(&mut self, key: String, value: String) {
        println!("inserted: {} => {}", key, value);
        self.map.insert(key, value);
    }

    //get a value (returns an option because the key might not exist)
    fn get(&self, key: &String) -> Option<&String> {
        //note: this returns option<&String> automatically
        self.map.get(key)
    }

    //delete a value
    fn delete(&mut self, key: &String) {
        self.map.remove(key);
        println!("deleted: {}", key);
    }
}

fn main() {
    let mut db = Database::new();

    loop {
        println!("\nRedis Lite: [1] Insert [2] Get [3] Delete [4] Quit");

        //todo:
        //1. create a string buffer for input
        let mut input = String::new();

        //2. read user input (stdin)
        io::stdin().read_line(&mut input).expect("failed to read line");
        let choice = input.trim();

        //3. match on the input:
        match choice {
            "1" => {
                println!("enter key:");
                let mut k = String::new();
                io::stdin().read_line(&mut k).expect("failed to read");

                println!("enter value:");
                let mut v = String::new();
                io::stdin().read_line(&mut v).expect("failed to read");

                //we use .trim().to_string() to remove the "enter" key newline
                db.insert(k.trim().to_string(), v.trim().to_string());
            }
            "2" => {
                println!("enter key to get:");
                let mut k = String::new();
                io::stdin().read_line(&mut k).expect("failed to read");
                let key_trimmed = k.trim().to_string();

                match db.get(&key_trimmed) {
                    Some(value) => println!("value is: {}", value),
                    None => println!("key not found!"),
                }
            }
            "3" => {
                println!("enter key to delete:");
                let mut k = String::new();
                io::stdin().read_line(&mut k).expect("failed to read");
                let key_trimmed = k.trim().to_string();

                db.delete(&key_trimmed);
            }
            "4" => {
                println!("goodbye!");
                break;
            }
            
            _ => println!("invalid choice, please try again."),
        }
    }
}