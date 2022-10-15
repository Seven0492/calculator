use std::env;
use std::path::Path;

use path_absolutize::*;

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

    // Get the absolute path to the current working directory
    let dir1 = Path::new(".");
    let dir2 = dir1.absolutize().unwrap();
    let working_directory = dir2.to_str().unwrap();

    // 'i' which means 'first argument'
    let mut i = 0;

    // Check if there's a weird starter argument unnacounted for
    if args[0] == working_directory || !args[0].parse::<f64>().is_ok() {
        if args.len() > 4 {
            println!("Not enough arguments. Please execute the program without any arguments for documentation");
        }

        i = 1;
    } else if args.len() > 3 {
        println!("Not enough arguments. Please execute the program without any arguments for documentation");
    }

    if args.is_empty() || args.len() >= 1 {
        println!(
            "Pass in a single basic mathematical operation to be made.

Options:
    Addition `2 + 4`,
    Substract `2 - 4`,
    Multiplication `2 x 4` but not `2 * 4` in Linux due to issues '*' creates,
    Division `2 / 4`.

For example: `./executable.exe 1 + 1` or `./executable.exe 2 x 4`\n"
        );
    // If a number
    } else if args[i].parse::<f64>().is_ok() {
        let mut numbers = NumSet {
            num1: args[i].clone(),
            num2: args[i + 2].clone(),
        };

        let mut symbol = args[i + 1].clone();

        // Debugging
        // println!("{}", symbol);

        if symbol == "+" {
            println!(
                "{}",
                numbers.add(numbers.num1.clone(), numbers.num2.clone())
            );
        } else if symbol == "-" {
            println!(
                "{}",
                numbers.substract(numbers.num1.clone(), numbers.num2.clone())
            );
        } else if symbol.to_lowercase() == "*" || symbol.to_lowercase() == "x" {
            println!(
                "{}",
                numbers.multiplicate(numbers.num1.clone(), numbers.num2.clone())
            );
        } else if symbol == "/" {
            println!(
                "{}",
                numbers.divide(numbers.num1.clone(), numbers.num2.clone())
            );
        } else if symbol.parse::<f64>().is_ok() {
            println!(
                "{}",
                numbers.multiplicate(numbers.num1.clone(), symbol.clone())
            );
        } else if i == 0 {
            i = 1;

            numbers.num1 = args[i].clone();
            numbers.num2 = args[i + 2].clone();

            symbol = args[i + 1].clone();

            if symbol == "+" {
                println!(
                    "{}",
                    numbers.add(numbers.num1.clone(), numbers.num2.clone())
                );
            } else if symbol == "-" {
                println!(
                    "{}",
                    numbers.substract(numbers.num1.clone(), numbers.num2.clone())
                );
            } else if symbol.to_lowercase() == "*" || symbol.to_lowercase() == "x" {
                println!(
                    "{}",
                    numbers.multiplicate(numbers.num1.clone(), numbers.num2.clone())
                );
            } else if symbol == "/" {
                println!(
                    "{}",
                    numbers.divide(numbers.num1.clone(), numbers.num2.clone())
                );
            } else if symbol.parse::<f64>().is_ok() {
                println!(
                    "{}",
                    numbers.multiplicate(numbers.num1.clone(), symbol.clone())
                );
            } else {
                panic!("Wrong formatting! Please execute the program without any arguments for documentation");
            }
        } else {
            panic!("Wrong formatting! Please execute the program without any arguments for documentation");
        }
    } else {
        println!(
            "Pass in a single basic mathematical operation to be made.

Options:
    Addition `2 + 4`,
    Substract ` 2 - 4`,
    Multiplication `2 * 4` or `2 x 4`,
    Division `2 / 4`.

For example: `./executable.exe 1 + 1` or `./executable.exe 2 x 4`\n"
        );
    }
}
