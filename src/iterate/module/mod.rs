mod doc;

pub struct NumSet {
    num1: String,
    symbol: String,
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

pub fn make_numset(args: &Vec<String>, i: &usize) -> NumSet {
    if args[i + 1].parse::<f64>().is_ok() {
        let numbers = NumSet {
            num1: args[i + 0].clone(),
            symbol: String::new(),
            num2: args[i + 1].clone(),
        };
        numbers
    } else {
        let numbers = NumSet {
            num1: args[i + 0].clone(),
            symbol: args[i + 1].clone(),
            num2: args[i + 2].clone(),
        };
        numbers
    }
}

pub fn operations(args: &Vec<String>, mut i: usize) -> bool {
    let set = make_numset(&args, &i);

    if set.symbol == "+" {
        println!("{}", set.add(set.num1.clone(), set.num2.clone()));
        true
    } else if set.symbol == "-" {
        println!("{}", set.substract(set.num1.clone(), set.num2.clone()));
        true
    } else if set.symbol.to_lowercase() == "*" || set.symbol.to_lowercase() == "x" {
        println!("{}", set.multiplicate(set.num1.clone(), set.num2.clone()));
        true
    } else if set.symbol == "/" {
        println!("{}", set.divide(set.num1.clone(), set.num2.clone()));
        true
    } else if set.symbol == String::new() {
        println!("{}", set.multiplicate(set.num1.clone(), set.num2.clone()));
        true
    } else if i == 0 {
        i = 1;

        // Redo but with an updated first argument setting
        if operations(&args, i) {
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
