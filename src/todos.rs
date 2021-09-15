extern crate serde;
extern crate serde_json;

pub mod structured {
    use std::io::Write;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Todo {
        name: String,
        description: String,
        done: bool
    }

    impl Todo {
        fn to_json(&self) -> String {
            let todo = generate_todo();
            return serde_json::to_string_pretty(&todo).unwrap();
        }

        fn to_file(&self) {
            let path_name = "./output/Todo.json";
            let prefix = std::path::Path::new(path_name).parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();

            let mut file = std::fs::File::create(path_name).expect("File creation failed!");
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
        let my_todo = generate_todo();
        my_todo.to_file();
        println!("I have task:\n- Name: {}\n- Description: {}\n- Done: {}", my_todo.name, my_todo.description, my_todo.done);
    }
}