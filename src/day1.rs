mod utils;

pub fn day1() {
    println!("Run day 1");
    let lines = utils::input_readers::read_numbers_as_vector("day1.txt");
    let mut first = 0;
    let mut second = 1;
    let mut third = 2;
    while first < lines.len() {
        second = first + 1;
        let mut found = false;
        while second < lines.len() {
            if (lines[first] + lines[second]) < 2020 {
                third = check_rest_of_vector(second, 2020 - (lines[first] + lines[second]), &lines);
                if third < lines.len() {
                    found = true;
                    break;
                }
            }
            second = second + 1;
        }
        if found {
            break;
        }
        first = first + 1;
    }
    println!(
        "The values {}, {}, {}, total is {}, product is {}",
        lines[first],
        lines[second],
        lines[third],
        lines[first] + lines[second] + lines[third],
        lines[first] * lines[second] * lines[third]
    );
}

fn check_rest_of_vector(index: usize, value: u32, lines: &Vec<u32>) -> usize {
    let mut counter = index + 1;
    while counter < lines.len() {
        if lines[counter] == value {
            break;
        } else {
            counter = counter + 1;
        }
    }
    counter
}
