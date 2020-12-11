mod utils;

pub fn day5() {
    println!("Start day 5");
    let lines: Vec<String> = utils::input_readers::read_strings_as_vector("day5.txt");

    let mut seat_map: [[bool; 8]; 128] = [[false; 8]; 128];
    let mut seat_ids: [[usize; 8]; 128] = [[0; 8]; 128];

    let mut highest = 0;
    for line in lines {
        let (row, column, id) = utils::input_readers::reduce_seat_assignment(&line);
        seat_map[row][column] = true;
        seat_ids[row][column] = id;
        if id > highest {
            highest = id;
        }
    }
    println!("The highest id is {}", highest);

    let mut row_count = 0;
    let mut seat_count;
    for row in seat_map.iter() {
        seat_count = 0;
        for seat in row.iter() {
            if !seat {
                println!(
                    "Seat {}, {} with ID {} is empty",
                    row_count,
                    seat_count,
                    (row_count * 8) + seat_count
                );
            }
            seat_count = seat_count + 1;
        }
        row_count = row_count + 1;
    }
}
