use convert_case::{Case, Casing};
use csv as crate_csv;
extern crate prettytable;
use prettytable::Table;
use slug;
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc;
use std::{error::Error, io, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    // Input-Recieving thread
    let input_thread = thread::spawn(move || loop {
        let (operation, data) = match get_input() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        tx.send((operation, data)).unwrap();
    });

    // Processing thread
    let processing_thread = thread::spawn(move || loop {
        let (parsed_op, data) = rx.recv().unwrap();
        let output = match parsed_op {
            Operation::Lowercase => lowercase(data),
            Operation::Uppercase => uppercase(data),
            Operation::Replace => replace(data),
            Operation::Slugify => slugify(data),
            Operation::Snake => snake(data),
            Operation::Camel => camel(data),
            Operation::Csv => csv(data),
        };

        match output {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("{}", e),
        };
    });
    input_thread.join().unwrap();
    processing_thread.join().unwrap();
}

#[derive(Debug)]
enum Operation {
    Lowercase,
    Uppercase,
    Replace,
    Slugify,
    Snake,
    Camel,
    Csv,
}
#[derive(Debug)]
struct ParseOperationError;
impl std::fmt::Display for ParseOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Invalid command.")
    }
}
impl std::error::Error for ParseOperationError {}
impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(action: &str) -> Result<Self, Self::Err> {
        match action {
            "lowercase" => Ok(Operation::Lowercase),
            "uppercase" => Ok(Operation::Uppercase),
            "replace" => Ok(Operation::Replace),
            "slugify" => Ok(Operation::Slugify),
            "snake" => Ok(Operation::Snake),
            "camel" => Ok(Operation::Camel),
            "csv" => Ok(Operation::Csv),
            _ => Err(ParseOperationError),
        }
    }
}

fn csv(data: String) -> Result<String, Box<dyn Error>> {
    let mut rdr = crate_csv::Reader::from_path(Path::new(&data))?;
    let table = Table::from_csv(&mut rdr);

    match table.print_tty(false) {
        Ok(n) => Ok(format!("{} lines printed", n)),
        Err(e) => Err(Box::new(e)),
    }
}

fn get_input() -> Result<(Operation, String), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let (op_string, data) = input
        .trim()
        .split_once(' ')
        .ok_or("Error: failed to parse data.")?;
    // .expect("Need space as delimiter to parse operation string");
    let op_result = op_string.parse::<Operation>();
    match op_result {
        Ok(op) => Ok((op, data.to_string())),
        Err(err) => Err(err.into()),
    }
}

fn lowercase(data: String) -> Result<String, Box<dyn Error>> {
    Ok(data.to_lowercase())
}

fn uppercase(data: String) -> Result<String, Box<dyn Error>> {
    Ok(data.to_uppercase())
}

fn replace(data: String) -> Result<String, Box<dyn Error>> {
    Ok(data.replace(' ', ""))
}

fn slugify(data: String) -> Result<String, Box<dyn Error>> {
    Ok(slug::slugify(data))
}

fn snake(data: String) -> Result<String, Box<dyn Error>> {
    Ok(data.to_case(Case::Snake))
}

fn camel(data: String) -> Result<String, Box<dyn Error>> {
    Ok(data.to_case(Case::Camel))
}
