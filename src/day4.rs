mod utils;

pub fn day4() {
    println!("Start day 4");
    let passports = utils::input_readers::read_strings_as_vector("day4.txt");
    let mut count = 0;
    for passport in passports {
        if passport.is_valid() {
            count = count + 1;
        }
    }
    println!("There are {} Valid passports.", count);
}
