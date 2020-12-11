mod utils;

pub fn day6() {
    println!("Start day 6");
    let lines = utils::input_readers::read_strings_as_vector("day6.txt");

    let count = utils::input_readers::count_group_count(&lines);
    println!("The count is {}", count);
}
