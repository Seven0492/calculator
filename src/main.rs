use std::env;
use std::path::Path;

use path_absolutize::*;

mod doc;

struct NumSet {
    num1: String,
    num2: String,
}

impl NumSet {
    pub fn add(&self, num1: String, num2: String) -> f64 {
        num1.parse::<f64>().unwrap() + num2.parse::<f64>().unwrap()
    }

    pub fn substract(&self, num1: String, num2: String) -> f64 {
        num1.parse::<f64>().unwrap() - num2.parse::<f64>().unwrap()
    }

    pub fn multiplicate(&self, num1: String, num2: String) -> f64 {
        num1.parse::<f64>().unwrap() * num2.parse::<f64>().unwrap()
    }

    pub fn divide(&self, num1: String, num2: String) -> f64 {
        num1.parse::<f64>().unwrap() / num2.parse::<f64>().unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Debugging
    // println!("Variable args: {:?}", args);
    // println!("Lenght of the args variable: '{}'", args.len());
    // EOD (Standing for: 'End Of Debugging')

    // Get the absolute path to the current working directory
    let dir1 = Path::new(".");
    let dir2 = dir1.absolutize().unwrap();
    let working_directory = dir2.to_str().unwrap();

    // Debugging
    // println!("Variable working_directory: '{}'", working_directory);
    // EOD

    // 'i' which means 'first argument'
    let mut i = 0;

    // Check if there's a weird starter argument unnacounted for
    if args[0] == working_directory
        || args[0] == "target/debug/calculator"
        || args[0] == "target/release/calculator"
        || !args[0].parse::<f64>().is_ok()
    {
        if args.len() > 4 {
            println!("Not enough arguments. Please execute the program without any arguments for documentation");
        }

        i = 1;

        // Debugging
        // println!(
        // "Var i: '{}' (From checking for a weird starter argument)",
        // i
        // );
        // EOD
    } else if args.len() > 3 {
        println!("Not enough arguments. Please execute the program without any arguments for documentation");

        // Debugging
        // println!("Variable i: '{}' (From args.len() > 3)", i);
        // EOD
    }

    if args.len() <= 1 {
        doc::help();

        // Debugging
        // println!("Variable i: '{}' (From args.len() <= 1)", i);
        // EOD

        // If first argument is a number
    } else {
        calculate(args, i);
    }
}

fn calculate(args: Vec<String>, i: usize) {
    if args[i].parse::<f64>().is_ok() {
        operations(&args, &i);
    } else {
        doc::wrong_formating(1);
        println!("\nDocumentation:\n\n");
        doc::help();
    }
}

fn make_numset(args: &Vec<String>, i: &usize) -> NumSet {
    if args[i + 1].parse::<f64>().is_ok() {
        let numbers = NumSet {
            num1: args[i + 0].clone(),
            num2: args[i + 1].clone(),
        };
        numbers
    } else {
        let numbers = NumSet {
            num1: args[i + 0].clone(),
            num2: args[i + 2].clone(),
        };
        numbers
    }
}

fn operations(args: &Vec<String>, mut i: &usize) -> bool {
    let numbers = make_numset(&args, &i);
    let symbol = args[i + 1].clone();

    if symbol == "+" {
        println!(
            "{}",
            numbers.add(numbers.num1.clone(), numbers.num2.clone())
        );
        true
    } else if symbol == "-" {
        println!(
            "{}",
            numbers.substract(numbers.num1.clone(), numbers.num2.clone())
        );
        true
    } else if symbol.to_lowercase() == "*" || symbol.to_lowercase() == "x" {
        println!(
            "{}",
            numbers.multiplicate(numbers.num1.clone(), numbers.num2.clone())
        );
        true
    } else if symbol == "/" {
        println!(
            "{}",
            numbers.divide(numbers.num1.clone(), numbers.num2.clone())
        );
        true
    } else if symbol.parse::<f64>().is_ok() {
        println!(
            "{}",
            numbers.multiplicate(numbers.num1.clone(), symbol.clone())
        );
        true
    } else if i == &0 {
        i = &1;

        // Redo but with an updated first argument setting
        if operations(&args, &i) {
            true
        } else {
            doc::wrong_formating(2);
            false
        }
    } else {
        doc::wrong_formating(2);
        false
    }
}
