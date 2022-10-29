mod doc;
pub mod module;

pub struct NumSet {
    num1: String,
    symbol: String,
    num2: String,
}

pub fn run(args: &Vec<String>, i: &usize) {
    let mut search_for_symbol = 0;

    let mut set = NumSet {
        num1: String::new(),
        symbol: String::new(),
        num2: String::new(),
    };

    let mut passed_over_initial_block = 0;

    for n in args.iter() {
        if i > &0 || &passed_over_initial_block != i {
            passed_over_initial_block += 1;

            continue;
        }

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

                if set.num1 == "" {
                    set.num1 = n.clone();
                } else if set.num2 == "" {
                    set.num2 = n.clone();
                } else {
                    set.num1 = n.clone();
                    let _ = set.num2 == String::new();
                }

                continue;
            }
        } else {
            if search_for_symbol == 1 {
                module::operations(
                    &vec![set.num1.clone(), set.symbol.clone(), set.num2.clone()],
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
