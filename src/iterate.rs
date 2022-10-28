mod doc;
mod main;

struct NumSet {
    num1: String,
    symbol: String,
    num2: String,
}

pub fn run(args: &Vec<String>, i: &usize) {
    let mut index = 0;
    let mut search_for_symbol = 0;

    let mut numbers = Numset { num1: String::new(), symbol: String::new(), num2: String::new() };

    if i == 0 {
        for n in args.iter().unwrap() {
            index += 1;

            if n.parse::<f64>.is_ok() {
                if search_for_symbol > 0 {
                    doc::wrong_formating(1);

                    // Debugging
                    println!("search_for_symbol: '{}'\n", search_for_symbol);
                    // EOD

                    doc::help();

                    exit;
                } else {
                    search_for_symbol = 1;

                    if numbers.num1 == "" {
                        numbers.num1 = n;
                    } else if numbers.num2 == "" {
                        numbers.num2 = n;
                    } else {
                        numbers.num1 = n;
                        numbers.num2 == String::new();
                    }

                    continue;
                }
            } else {
                if search_for_symbol == 1 {
                    main::operations(&vec![numbers.num1, numbers.symbol, numbers.num2], &0);
                } else {
                    doc::wrong_formating(1);

                    // Debugging
                    println!("search_for_symbol: '{}'\n", search_for_symbol);
                    // EOD

                    doc::help();

                    exit;
                }
            }
        }
    }
}
