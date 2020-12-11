pub mod input_readers {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };
    pub fn read_strings_as_vector(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("Could not read");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .map(|l| l.parse::<String>().unwrap())
            .collect()
    }
    pub fn count_group_count(lines: &Vec<String>) -> usize {
        let mut alphabets: [bool; 26] = [false; 26];
        let mut count = 0;
        for line in lines {
            if line.trim().len() == 0 {
                for alphabet in alphabets.iter() {
                    if *alphabet {
                        count = count + 1;
                    }
                }
                alphabets = [false; 26];
            }
            for literal in line.chars() {
                alphabets[((literal as i8) as usize) - 97] = true;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {

    use super::input_readers;

    #[test]
    fn check_count() {
        let lines = input_readers::read_strings_as_vector("day6.txt");
        assert_eq!(input_readers::count_group_count(&lines), 11);
    }
}
