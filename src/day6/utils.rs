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
        let mut group_choices: [bool; 26] = [true; 26];
        let mut line_choices: [bool; 26] = [false; 26];
        let mut pristine = true;
        let mut count = 0;
        for line in lines {
            if !pristine {
                for i in 0..26 {
                    group_choices[i] = group_choices[i] && line_choices[i];
                    println!("Literal {} is {}", i, group_choices[i]);
                }
                pristine = false;
            }
            if line.trim().len() == 0 {
                println!("Starting count is {}", count);
                for choice in group_choices.iter() {
                    if *choice {
                        count = count + 1
                    }
                }
                group_choices = [true; 26];
                pristine = true;
                println!("Count in this group is {}", count);
            }
            line_choices = [false; 26];
            for literal in line.chars() {
                line_choices[((literal as i8) as usize) - 97] = true;
                println!("Found {}", literal);
                pristine = false;
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
        let lines = input_readers::read_strings_as_vector("day6_test.txt");
        assert_eq!(input_readers::count_group_count(&lines), 6);
    }
}
