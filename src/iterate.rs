mod doc;
pub mod module;

pub struct NumSet {
    num1: String,
    symbol: String,
    num2: String,
}

pub fn run(args: &Vec<String>, i: &usize) {
    let mut search_for_symbol = 0;

    let mut numbers = NumSet {
        num1: String::new(),
        symbol: String::new(),
        num2: String::new(),
    };

    if i == &0 {
        for n in args.iter() {
            if n.parse::<f64>().is_ok() {
                if search_for_symbol > 0 {
                    doc::wrong_formating(1);

                    // Debugging
                    println!("search_for_symbol: '{}'\n", search_for_symbol);
                    // EOD

                    doc::help();

                    return;
                } else {
                    search_for_symbol = 1;

                    if numbers.num1 == "" {
                        numbers.num1 = n.clone();
                    } else if numbers.num2 == "" {
                        numbers.num2 = n.clone();
                    } else {
                        numbers.num1 = n.clone();
                        let _ = numbers.num2 == String::new();
                    }

                    continue;
                }
            } else {
                if search_for_symbol == 1 {
                    module::operations(
                        &vec![
                            numbers.num1.clone(),
                            numbers.symbol.clone(),
                            numbers.num2.clone(),
                        ],
                        &0,
                    );

                    continue;
                } else {
                    doc::wrong_formating(1);

                    // Debugging
                    println!("search_for_symbol: '{}'\n", search_for_symbol);
                    // EOD

                    doc::help();

                    return;
                }
            }
        }
    }
}
