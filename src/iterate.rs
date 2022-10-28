mod doc;

struct NumSet {
    num1: String,
    num2: String,
}

pub fn run(array: &Vec<String>, i: &usize) {
    let mut index = 0;
    let mut search_for_symbol = 0;

    let mut nums: Numset = Numset { num1: 0, num2: 0 };

    if i == 0 {
        for n in array.iter().unwrap() {
            index += 1;

            if n.parse::<f64>.is_ok() {
                if search_for_symbol > 0 {
                    doc::help();
                    todo!("Iterator to add multiple operations functionality");
                } else {
                    search_for_symbol = 1;

                    todo!("Iterator to add multiple operations functionality");

                    continue;
                }
            } else { todo!("Iterator to add multiple operations functionality"); }
        }
    }
}
