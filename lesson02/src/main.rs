use slug::{self, slugify};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "No parameter provided");

    println!("Enter string:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    println!("Content of args: {:?}", args);

    if args[1] == "lowercase" {
        println!("{}", input.to_lowercase());
    } else if args[1] == "uppercase" {
        println!("{}", input.to_uppercase());
    } else if args[1] == "no-spaces" {
        println!("{}", input.replace(" ", ""));
    } else if args[1] == "slugify" {
        println!("{}", slugify(&input));
    }
}
