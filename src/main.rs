mod todos;
mod introductions;

#[macro_use] extern crate serde_derive;

fn main() {
    introductions::input_name_welcome();
    print_breakline();
    introductions::parametrized_hello("World".to_string());
    print_breakline();
    todos::structured::sumarize_todo();
}

fn print_breakline() {
    println!("===========================================================");
}