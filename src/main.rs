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
        //todo: use the .insert() method on your hashmap
        //print "inserted: [key] => [value]"
    }

    //get a value (returns an option because the key might not exist)
    fn get(&self, key: &String) -> Option<&String> {
        //todo: use the .get() method on your hashmap
        //note: this returns option<&String> automatically
        self.map.get(key)
    }

    //delete a value
    fn delete(&mut self, key: &String) {
        //todo: use the .remove() method
        //print "deleted: [key]"
    }
}

fn main() {
    let mut db = Database::new();

    loop {
        println!("\nRedis Lite: [1] Insert [2] Get [3] Delete [4] Quit");

        //todo:
        //1. create a string buffer for input
        //2. read user input (stdin)
        //3. match on the input:
        //    -if "1": ask for key, ask for value, call db.insert()
        //    -if "2": ask for key, call db.get(). match on the result (some or none)
        //    -if "3": ask for key, call db.delete()
        //    -if "4": break loop
    }
}