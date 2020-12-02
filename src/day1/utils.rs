pub mod input_readers {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };
    pub fn read_numbers_as_vector(filename: impl AsRef<Path>) -> Vec<u32> {
        println!("Read as vector of number");
        let file = File::open(filename).expect("Could not read");
        let buf = BufReader::new(file);
        buf.lines().map(|l| l.expect("Could not parse line")).map(|l| l.parse::<u32>().unwrap()).collect()
    }
}
