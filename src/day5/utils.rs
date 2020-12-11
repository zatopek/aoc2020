pub mod input_readers {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };
    pub fn read_strings_as_vector(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("Could not read");
        let buf = BufReader::new(file);
        buf.lines().map(|l| l.expect("Could not parse line")).map(|l| l.parse::<String>().unwrap()).collect()
    }
    pub fn reduce_seat_assignment(seat: &String) -> (usize, usize, usize) {
        let commands: Vec<char> = seat.chars().collect();
        let mut row_low = 0;
        let mut row_high = 127;
        let mut row_size = 128;
        let mut column_low = 0;
        let mut column_high = 7;
        let mut column_size = 8;

        for command in commands {
            if command.eq_ignore_ascii_case(&'F') {
                row_high = row_high - row_size / 2;
                row_size = row_size / 2;
            } else if command.eq_ignore_ascii_case(&'B') {
                row_low = row_low + (row_size / 2);
                row_size = row_size / 2;
            } else if command.eq_ignore_ascii_case(&'L') {
                column_high = column_high - column_size / 2;
                column_size = column_size / 2;
            } else if command.eq_ignore_ascii_case(&'R') {
                column_low = column_low + column_size / 2;
                column_size = column_size / 2;
            }
        }
        (row_high, column_high, ((row_high * 8) + column_high))
    }
}

#[cfg(test)]
mod test {

    use super::input_readers;

    #[test]
    fn check_traversal() {
        assert_eq!(
            input_readers::reduce_seat_assignment(&String::from("FBFBBFFRLR")),
            (44, 5, 357)
        );
        assert_eq!(
            input_readers::reduce_seat_assignment(&String::from("BFFFBBFRRR")),
            (70, 7, 567)
        );
        assert_eq!(
            input_readers::reduce_seat_assignment(&String::from("FFFBBBFRRR")),
            (14, 7, 119)
        );
        assert_eq!(
            input_readers::reduce_seat_assignment(&String::from("BBFFBBFRLL")),
            (102, 4, 820)
        );
    }
}
