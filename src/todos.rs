extern crate serde;
extern crate serde_json;

pub mod structured {
    use std::io::Write;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Todo {
        id: String,
        name: String,
        description: String,
        done: bool
    }

    // Create or implement structure functionality
    impl Todo {

        // To implement method for structure we need to pass &self parameter
        fn to_json(&self) -> String {
            return serde_json::to_string_pretty(&self).unwrap();
        }

        // To implement method for structure we need to pass &self parameter
        fn to_file(&self) {
            let path_name = "./output/Todo.json";

            // Check if directory exists
            let parent_folder = std::path::Path::new(path_name).parent().unwrap();

            // Create directory if not present
            std::fs::create_dir_all(parent_folder).unwrap();

            let mut file = std::fs::File::create(path_name).expect("File creation failed!");
            file.write_all(self.to_json().as_bytes()).expect("File writing failed!");

            println!("===========================================================");
            println!("Data successfully written to file!");
            println!("Please check output file: {}", path_name);
         }
    }

    fn new_todo(name: String, description: String, done: bool) -> Todo {
        return Todo{
            id: Uuid::new_v4().to_string(),
            name,
            description,
            done
        };
    }

    fn generate_todo() -> Todo {
        let todo = new_todo(
            get_input("What is task name".to_string()),
            get_input("How would you describe it".to_string()),
            false
        );
         return todo;
    }

    // Function do all tasks together: create task, export task to file and print task to console
    pub fn summarize_todo() {
        let my_todo = generate_todo();
        my_todo.to_file();

        println!("===========================================================");
        println!("I have task:");
        println!("- ID: {}", my_todo.id);
        println!("- Name: {}", my_todo.name);
        println!("- Description: {}", my_todo.description);
        println!("- Done: {}", my_todo.done);
    }

    // Create input prompt for user
    fn get_input(console_text: String) -> String {
        let mut tmp_input = String::new();
        println!("{}:", console_text);
        std::io::stdin().read_line(&mut tmp_input).expect("Failed To read Input");
        return tmp_input.trim_end().to_string();
    }
}