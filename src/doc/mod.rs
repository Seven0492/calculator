pub fn help() {
    println!("\nHelp page:\n\n");
    println!(
        "Pass in a single basic mathematical operation to be made.

Options:
  Addition `2 + 4`,
  Substract ` 2 - 4`,
  Multiplication `2 x 4` but using '*' without quotes can create problems,
  Division `2 / 4`.

If you just type two numbers separated by spaces without any symbols,
it will default to multiplication.

For example: `./executable.exe 1.2 + 63.52` or `./executable.exe 2 x 4`\n"
    );
}

pub fn wrong_formating(n: usize) {
    if n == 1 || n > 3 || n < 1 {
        eprintln!("Wrong formatting!");
    } else if n == 2 {
        eprintln!(
            "Wrong formatting! Please execute the program without any arguments for documentation"
        );
    } else if n == 3 {
        eprintln!(
            "Wrong formatting! Please execute the program without any arguments for documentation"
        );
    }
}
