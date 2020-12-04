mod utils;

pub fn day3() {
    println!("Start day 3");
    let lines = utils::input_readers::read_values_as_2d_array("day3.txt");

    let count1 = count_trees_on_slope(&lines, 1, 1);
    println!("Total trees encountered are {}", count1);
    let count2 = count_trees_on_slope(&lines, 3, 1);
    println!("Total trees encountered are {}", count2);
    let count3 = count_trees_on_slope(&lines, 5, 1);
    println!("Total trees encountered are {}", count3);
    let count4 = count_trees_on_slope(&lines, 7, 1);
    println!("Total trees encountered are {}", count4);
    let count5 = count_trees_on_slope(&lines, 1, 2);
    println!("Total trees encountered are {}", count5);

    println!(
        "Product of all the counts are {}",
        count1 * count2 * count3 * count4 * count5
    );
}

pub fn count_trees_on_slope(forest: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut startingx = 0;
    let mut startingy = 0;
    let mut count = 0;
    while startingy < forest.len() {
        if forest[startingy][startingx] == '#' {
            count = count + 1
        }

        startingx = (startingx + right) % forest[startingy].len();
        startingy = startingy + down;
    }
    count
}
