pub mod input_readers {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };
    pub fn read_values_as_2d_array(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
        let file = File::open(filename).expect("Could not read");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).map(|l| l.parse::<String>().unwrap()).collect();
        let mut result = Vec::new();
        for line in lines {
            let char_array: Vec<char> = line.trim().chars().collect();
            result.push(char_array);
        }
        result
    }
}
