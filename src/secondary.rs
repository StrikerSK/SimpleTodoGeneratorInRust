use std::io::Write;

extern crate serde;
extern crate serde_json;

pub mod introduction {
    pub fn other_fn() {
        println!("Hello from other pacakge");
    }
}

pub mod structured {
    use std::io::Write;


    #[derive(Serialize, Deserialize, Debug)]
    pub struct Todo {
        name: String,
        description: String,
        done: bool
    }

    impl Todo {
        // fn toggle_done(&mut self) {
        //     self.done = !self.done
        // }

        fn to_json(&self) -> String {
            let todo = generate_todo();
            return serde_json::to_string(&todo).unwrap();
        }

        fn replace_description(&mut self, new_description: String) {
            self.description = new_description;
        }

        fn to_file(&self) {
            let mut file = std::fs::File::create("TodoInFile.json").expect("File creation failed!");
            file.write_all(self.to_json().as_bytes()).expect("File writing failed!");
            println!("Data written to file!" );
         }
    }

    fn new_todo(name: String, description: String, done: bool) -> Todo {
        return Todo{
            name: name,
            description: description,
            done: done
        };
    }

    fn generate_todo() -> Todo {
        let todo = new_todo(
            "Create some first Rust code".to_string(),
            "Just to implement stuff".to_string(),
            true
        );
         return todo;
    }

    pub fn sumarize_todo() {
        let mut my_todo = generate_todo();
        my_todo.to_file();
        println!("I have task:\n- Name: {}\n- Description: {}\n- Done: {}", my_todo.name, my_todo.description, my_todo.done);
    }
}