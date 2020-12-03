mod utils;

pub fn day2() {
    println!("Run day 1");
    let lines = utils::input_readers::read_strings_as_vector("day2.txt");
    let mut valid_passwords = 0;
    for line in lines {
        if is_valid(line) {
            valid_passwords = valid_passwords + 1;
        }
    }
    println!("Total valid password are {}", valid_passwords);
}

fn is_valid(line: String) -> bool {
    let split: Vec<&str> = line.split(":").collect();
    let rule = split[0].trim();
    let rule_split: Vec<&str> = rule.split(" ").collect();
    let range = rule_split[0].trim();
    let range_split: Vec<&str> = range.split("-").collect();
    let min = range_split[0].trim().parse::<usize>().unwrap();
    let max = range_split[1].trim().parse::<usize>().unwrap();
    let character = rule_split[1].trim();
    let password = split[1].trim();
    let count = password.matches(character).count();
    count >= min && count <= max
}
