use std::collections::HashMap;
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    fn new() -> Result<Todo, std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(error) if error.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(error) => panic!("An error occurred: {}", error),
        }
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(file, &self.map)?;
        Ok(())
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    //  cargo run -- add "make coffee"
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("TODO saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}
