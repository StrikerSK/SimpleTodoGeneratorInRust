mod todos;

#[macro_use] extern crate serde_derive;

fn main() {
    todos::structured::summarize_todo();
}