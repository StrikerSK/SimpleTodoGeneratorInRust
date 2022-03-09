mod todos;
use chrono::{Utc};

#[macro_use] extern crate serde_derive;

fn main() {
    //Print current time in format RFC2822
    println!("Application run at: {}", Utc::now().to_rfc2822());
    println!("===========================================================");
    todos::structured::summarize_todo();
}