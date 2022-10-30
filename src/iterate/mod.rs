mod doc;
pub mod module;

#[derive(Debug)]
pub struct NumSet {
    num1: String,
    symbol: String,
    num2: String,
}

pub fn run(args: Vec<String>, i: usize, passed_over_initial_block: usize) {
    if cfg!(debug_assertions) {
        println!("Initial args variable: {:?}", &args);
    }

    let mut search_for_symbol = 0;

    let mut set = NumSet {
        num1: String::new(),
        symbol: String::new(),
        num2: String::new(),
    };

    let mut index = 0;

    for n in args.iter() {
        // Debugging
        if cfg!(debug_assertions) {
            println!("Variable n: {}", &n);
        }
        // EOD

        index += 1;

        // Debugging
        if cfg!(debug_assertions) {
            println!("args.len(): {}", args.len());
            println!("Index: {}", &index);
        }
        // EOD

        if passed_over_initial_block != i {
            // Debugging
            if cfg!(debug_assertions) {
                println!("Restarting loop");
            }
            // EOD

            let mut new_args = args.clone();

            new_args.remove(0);

            run(new_args, i, passed_over_initial_block + 1);

            return;
        }

        if n.parse::<f64>().is_ok() {
            if search_for_symbol > 0 {
                if cfg!(debug_assertions) {
                    eprintln!("Searching for symbol but finding a number");
                }

                doc::wrong_formating(1);

                // Debugging
                if cfg!(debug_assertions) {
                    println!("search_for_symbol: '{}'", &search_for_symbol);
                }
                // EOD

                doc::help();

                return;
            } else {
                // Debugging
                if cfg!(debug_assertions) {
                    println!("Old search_for_symbol: '{}'", &search_for_symbol);
                }
                // EOD

                search_for_symbol = 1;

                // Debugging
                if cfg!(debug_assertions) {
                    println!("New search_for_symbol: '{}'", &search_for_symbol);
                }
                // EOD

                if set.num1 == "" {
                    // Debugging
                    if cfg!(debug_assertions) {
                        println!("Cloning 'n' into set.num1 which is: {}", &set.num1);
                    }
                    // EOD

                    set.num1 = n.clone();

                    // Debugging
                    if cfg!(debug_assertions) {
                        println!("New set.num1: {}", &set.num1);
                        println!("{:?}", &set);
                    }
                    // EOD
                } else if set.num2 == "" {
                    // Debugging
                    if cfg!(debug_assertions) {
                        println!("Cloning 'n' into set.num2 which is: {}", &set.num2);
                    }
                    // EOD

                    set.num2 = n.clone();

                    // Debugging
                    if cfg!(debug_assertions) {
                        println!("New set.num2: {}", &set.num2);
                        println!("{:?}", &set);
                    }
                    // EOD
                } else {
                    // Debugging
                    if cfg!(debug_assertions) {
                        println!("Cloning 'n' into set.num1 which is: {}", &set.num1);
                    }
                    // EOD

                    set.num1 = n.clone();

                    // Debugging
                    if cfg!(debug_assertions) {
                        println!("New set.num1: {}", &set.num1);
                        println!("Emptying set.num2: {}", &set.num2);
                        println!("Emptying set.symbol: {}", &set.symbol);
                    }
                    // EOD

                    set.num2 = String::new();
                    set.symbol = String::new();

                    // Debugging
                    if cfg!(debug_assertions) {
                        println!("New set.num2: {}", &set.num2);
                        println!("New set.symbol: {}", &set.num2);
                        println!("{:?}", &set);
                    }
                    // EOD
                }
            }

            if index % 3 == 0 {
                // Debugging
                if cfg!(debug_assertions) {
                    println!(
                        "Soon to be passed Vec<String>: {:?}",
                        vec![set.num1.clone(), set.symbol.clone(), set.num2.clone()]
                    );
                    println!("Passing into src/iterate/module/mod.rs::operations()\n");
                }
                // EOD

                module::operations(
                    vec![set.num1.clone(), set.symbol.clone(), set.num2.clone()],
                    0,
                );

                search_for_symbol = 0;

                continue;
            }
        } else {
            if set.num1 != "" && set.num2 != "" && set.symbol != "" && search_for_symbol == 1 {
                // Debugging
                if cfg!(debug_assertions) {
                    println!(
                        "Soon to be passed Vec<String>: {:?}",
                        vec![set.num1.clone(), set.symbol.clone(), set.num2.clone()]
                    );
                    println!("Passing into src/iterate/module/mod.rs::operations()\n");
                }
                // EOD

                module::operations(
                    vec![set.num1.clone(), set.symbol.clone(), set.num2.clone()],
                    0,
                );

                search_for_symbol = 0;

                continue;
            } else if set.num2 == "" && set.symbol == "" && set.num1 != "" && search_for_symbol == 1
            {
                // Debugging
                if cfg!(debug_assertions) {
                    println!("Old search_for_symbol: {}", &search_for_symbol);
                }
                // EOD

                search_for_symbol = 0;

                // Debugging
                if cfg!(debug_assertions) {
                    println!("New search_for_symbol: {}", &search_for_symbol);
                    println!("Cloning 'n' into empty set.symbol");
                }
                // EOD

                set.symbol = n.clone();

                // Debugging
                if cfg!(debug_assertions) {
                    println!("New set.symbol: {}", &set.symbol);
                    println!("{:?}", &set);
                }
                // EOD
            } else {
                doc::wrong_formating(1);

                // Debugging
                println!("search_for_symbol: '{}'\n", &search_for_symbol);
                // EOD

                doc::help();

                return;
            }

            if index % 3 == 0 {
                // Debugging
                if cfg!(debug_assertions) {
                    println!(
                        "Soon to be passed Vec<String>: {:?}",
                        vec![set.num1.clone(), set.symbol.clone(), set.num2.clone()]
                    );
                    println!("Passing into src/iterate/module/mod.rs::operations()\n");
                }
                // EOD

                module::operations(
                    vec![set.num1.clone(), set.symbol.clone(), set.num2.clone()],
                    0,
                );

                search_for_symbol = 0;
            }
        }
    }
}
