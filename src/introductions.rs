pub fn parametrized_hello(input: String) {
    println!("Hello {}!", input);
}
    
pub fn input_name_welcome() {
    let mut line = String::new();
    print!("Enter your name:\n");
    std::io::stdin().read_line(&mut line).expect("Failed To read Input");
    parametrized_hello(line.trim().to_string());
}
