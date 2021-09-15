extern crate serde;
extern crate serde_json;

pub mod introduction {
    pub fn other_fn() {
        println!("Hello from other pacakge");
    }
}

pub mod structured {

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

        fn to_json(&self) {
            let todo = generate_todo();
            let serialized = serde_json::to_string(&todo).unwrap();
            println!("Data = {}", serialized);
        }

        fn replace_description(&mut self, new_description: String) {
            self.description = new_description;
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
        my_todo.replace_description("Haha, I just changed it as paramtere!".to_string());
        my_todo.to_json();
        println!("I have task:\n- Name: {}\n- Description: {}\n- Done: {}", my_todo.name, my_todo.description, my_todo.done);
    }
}