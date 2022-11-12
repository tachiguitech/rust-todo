use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("./data/db.json")?;
        serde_json::to_writer_pretty(content, &self.map)?;
        Ok(())
    }

    fn new() -> Result<Todo, std::io::Error> {
        let list = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("./data/db.json")?;

        match serde_json::from_reader(list) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn show(&mut self) {
        for (key, val) in self.map.iter() {
            println!("{} {}", key, val);
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action.");
    let item = std::env::args().nth(2).expect("Please specify an action.");

    let mut todo = Todo::new().expect("Initialisation of db failed.");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo item saved."),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list.", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo item saved."),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "show" {
        todo.show();
    }
}
