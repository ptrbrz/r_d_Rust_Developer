use std::io;
fn main() {
    println!("Enter your name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    let name = name.trim();

    println!("How are you doing {}?", name);

    let mut mood = String::new();
    io::stdin()
        .read_line(&mut mood)
        .expect("Failed to read line.");

    let mood = mood.trim().to_lowercase();

    if mood == "great" || mood == "cool" {
        println!("That is great {}!", name);
    } else {
        println!("{}, I hope you will get better.", name);
    }
}
