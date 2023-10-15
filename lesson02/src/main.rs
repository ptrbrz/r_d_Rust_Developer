// two own transmutations
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

    println!("Content of args: {:?}", args);

    //     if args[1] == "lowercase" {
    //         println!("{}", input.to_lowercase());
    //     } else if args[1] == "uppercase" {
    //         println!("{}", input.to_uppercase());
    //     } else if args[1] == "no-spaces" {
    //         println!("{}", input.replace(" ", ""));
    //     } else if args[1] == "slugify" {
    //         println!("{}", slugify(&input));
    //     }
    match args[1].as_str() {
        "lowercase" => println!("{}", input.to_lowercase()),
        "uppercase" => println!("{}", input.to_uppercase()),
        "replace" => println!("{}", input.replace(" ", "")),
        "slugify" => println!("{}", slugify(&input)),
        _ => println!("Valid input parameter missing."),
    }
}
