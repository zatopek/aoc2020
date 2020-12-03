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
}
