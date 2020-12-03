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
    let exist = range_split[0].trim().parse::<usize>().unwrap();
    let not_exist = range_split[1].trim().parse::<usize>().unwrap();
    let character = rule_split[1].trim().parse::<char>().unwrap();
    let password: Vec<char> = split[1].trim().chars().collect();
    (password[exist-1] == character && password[not_exist-1] != character) || (password[exist-1] != character && password[not_exist-1] == character)
}
