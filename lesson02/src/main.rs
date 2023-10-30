use convert_case::{Case, Casing};
use csv as crate_csv;
use prettytable::Table;
use slug;
use std::io;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let op_string = env::args().nth(1);

    let output = match op_string.as_deref() {
        Some("lowercase") => lowercase(),
        Some("uppercase") => uppercase(),
        Some("replace") => replace(),
        Some("slugify") => slugify(),
        Some("snake") => snake(),
        Some("camel") => camel(),
        Some("csv") => csv(),
        Some(&_) => Err("ERROR: Operation parameter is not valid".into()),
        None => Err("ERROR: Missing operation parameter".into()),
    };

    match output {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e),
    };
    Ok(())
}

fn get_string() -> Result<String, Box<dyn Error>> {
    let string = loop {
        println!("Enter string:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();
        if !input.is_empty() {
            break input;
        }
    };
    Ok(string)
}

fn csv() -> Result<String, Box<dyn Error>> {
    let mut rdr = crate_csv::Reader::from_reader(io::stdin());
    let table = Table::from_csv(&mut rdr);

    match table.print_tty(false) {
        Ok(n) => Ok(format!("{} lines printed", n)),
        Err(e) => Err(Box::new(e)),
    }
}

fn lowercase() -> Result<String, Box<dyn Error>> {
    let input = get_string()?;
    Ok(input.to_lowercase())
}

fn uppercase() -> Result<String, Box<dyn Error>> {
    let input = get_string()?;
    Ok(input.to_uppercase())
}

fn replace() -> Result<String, Box<dyn Error>> {
    let input = get_string()?;
    Ok(input.replace(' ', ""))
}

fn slugify() -> Result<String, Box<dyn Error>> {
    let input = get_string()?;
    Ok(slug::slugify(input))
}

fn snake() -> Result<String, Box<dyn Error>> {
    let input = get_string()?;
    Ok(input.to_case(Case::Snake))
}

fn camel() -> Result<String, Box<dyn Error>> {
    let input = get_string()?;
    Ok(input.to_case(Case::Camel))
} //     todo!();
  // }

// let operation = match op_string {
//             Some(action) => match Operation::from_str(&action) {
//         Ok(op) => op,
//         Err(e) => return Err(e),
//     },
//             None => eprintln!("Error: No operation argument."),
// }
// let operation = env::args().nth(1).map_or(Ok(None), |arg| Operation::from_str(&arg).map_or_else(|e| eprintln!(e), |v| v ) );
// let operation = Operation::from_str(env::args().nth(1).ok_or_else());
// .and_then(|arg| arg.parse::<Operation>())?;
// let operation: Result<Operation, ParseOperationError> = env::args().map(|arg| Opertion::from_str(&arg));
// let operation: Operation = Operation::from_str(&env::args().collect().unwrap()).unwrap();
// let args: Vec<String> = env::args().collect();
// assert!(args.len() > 1, "No parameter provided");
// match operation {
//     Ok(operation) =>

// }
// let output_result = match operation.ok() {
//     Some(Operation::Lowercase) => lowercase(input),
//     Some(Operation::Uppercase) => uppercase(input),
//     Some(Operation::Replace) => replace(input),
//     Some(Operation::Slugify) => slugify(input),
//     Some(Operation::Snake) => snake(input),
//     Some(Operation::Camel) => camel(input),
//     // Some(Operation::Csv) => output_result = csv(input),
//     _ => Ok(()),
// };

// enum Operation {
//        Lowercase,
//        Uppercase,
//        Replace,
//        Slugify,
//        Snake,
//        Camel,
//        Csv,
//    }
//    struct ParseOperationError;
//    impl FromStr for Operation {
//        type Err = ParseOperationError;

//        fn from_str(action: &str) -> Result<Operation, Self::Err> {
//            match action {
//                "lowercase" => Ok(Operation::Lowercase),
//                "uppercase" => Ok(Operation::Uppercase),
//                "replace" => Ok(Operation::Replace),
//                "slugify" => Ok(Operation::Slugify),
//                "snake" => Ok(Operation::Snake),
//                "camel" => Ok(Operation::Camel),
//                // "csv" => Ok(Operation::Csv),
//                _ => Err(ParseOperationError),
//            }
//        }
//    };
