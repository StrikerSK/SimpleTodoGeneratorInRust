mod secondary;

#[macro_use] extern crate serde_derive;

fn main() {
    secondary::introduction::other_fn();
    input_name_welcome();
    parametrized_hello("World".to_string());
    secondary::structured::sumarize_todo();
}

fn parametrized_hello(input: String) {
    println!("Hello {}!", input);
}

fn input_name_welcome() {
    let mut line = String::new();
    print!("Enter your name:\n");
    std::io::stdin().read_line(&mut line).expect("Failed To read Input");
    parametrized_hello(line.trim().to_string());
}