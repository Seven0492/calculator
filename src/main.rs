use std::env;
use std::path::Path;

use path_absolutize::*;

mod doc;
mod iterate;

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
        // if args.len() > 4 {
        // println!("Not enough arguments. Please execute the program without any arguments for documentation");
        // }

        i = 1;

        // Debugging
        // println!(
        // "Var i: '{}' (From checking for a weird starter argument)",
        // i
        // );
        // EOD
    } else if args.len() > 3 {
        // Disable warning
        // println!("Too many arguments. Please execute the program without any arguments for documentation");

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
        iterate::module::operations(&args, &i);
    } else {
        doc::wrong_formating(1);
        println!("\nDocumentation:\n\n");
        doc::help();
    }
}
