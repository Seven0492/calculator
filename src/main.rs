use std::env;
use std::path::Path;

use path_absolutize::*;

mod doc;
mod iterate;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Debugging (Only compiles in debugging mode)
    if cfg!(debug_assertions) {
        println!("Variable args: {:?}", args);
        println!("Lenght of the args variable: '{}'", args.len());
    }
    // EOD (Standing for: 'End Of Debugging')

    // Get the absolute path to the current working directory
    let dir1 = Path::new(".");
    let dir2 = dir1.absolutize().unwrap();
    let working_directory = dir2.to_str().unwrap();

    // Debugging
    if cfg!(debug_assertions) {
        println!("Variable working_directory: '{}'", working_directory);
    }
    // EOD

    // 'i' which means 'first argument'
    let mut i = 0;

    // Check if there's a weird starter argument unnacounted for
    if args[0] == working_directory
        || args[0] == "target/debug/calculator"
        || args[0] == "target/release/calculator"
        || !args[0].parse::<f64>().is_ok()
    {
        i = 1;

        // Debugging
        if cfg!(debug_assertions) {
            println!(
                "Var i: '{}' (From checking for a weird starter argument)",
                i
            );
        }
        // EOD
    } else if args.len() > 3 {
        // Debugging
        if cfg!(debug_assertions) {
            println!("More than 3 arguments");
            println!("Variable i: '{}' (From args.len() > 3)", i);
        }
        // EOD
    }

    if args.len() <= 1 {
        doc::help();

        // Debugging
        if cfg!(debug_assertions) {
            println!("Variable i: '{}' (From args.len() <= 1)", i);
        }
        // EOD

        // If first argument is a number
    } else {
        calculate(args, i);
    }
}

fn calculate(args: Vec<String>, i: usize) {
    // If initial argument is a number
    if args[i].parse::<f64>().is_ok() {
        // The continue with operations
        iterate::module::operations(&args, &i);
    // Else warn about wrong formatting and print the help page
    } else {
        doc::wrong_formating(1);
        doc::help();
    }
}
