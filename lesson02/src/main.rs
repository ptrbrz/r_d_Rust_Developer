// two own transmutations
use convert_case::{Case, Casing};
use slug::slugify;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "No parameter provided");

    println!("Enter string:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let input = input.trim();

    match args.get(1).unwrap().as_str() {
        "lowercase" => println!("{}", input.to_lowercase()),
        "uppercase" => println!("{}", input.to_uppercase()),
        "replace" => println!("{}", input.replace(" ", "")),
        "slugify" => println!("{}", slugify(input)),
        "snake" => println!("{}", input.to_case(Case::Snake)),
        "camel" => println!("{}", input.to_case(Case::Camel)),
        _ => println!("Invalid parameter.",), // "camel" => println!("{}", input.to_case)
    }
}
