pub mod input_readers {
    use std::{
        fmt,
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };
    pub struct Passport {
        byr: String,
        iyr: String,
        eyr: String,
        hgt: String,
        hcl: String,
        ecl: String,
        pid: String,
        cid: String,
    }
    impl Passport {
        fn new() -> Passport {
            Passport {
                byr: String::from(""),
                iyr: String::from(""),
                eyr: String::from(""),
                hgt: String::from(""),
                hcl: String::from(""),
                ecl: String::from(""),
                pid: String::from(""),
                cid: String::from(""),
            }
        }
        fn set_byr(&mut self, byr: String) {
            self.byr = byr;
        }
        fn set_iyr(&mut self, iyr: String) {
            self.iyr = iyr;
        }
        fn set_eyr(&mut self, eyr: String) {
            self.eyr = eyr;
        }
        fn set_hgt(&mut self, hgt: String) {
            self.hgt = hgt;
        }
        fn set_hcl(&mut self, hcl: String) {
            self.hcl = hcl;
        }
        fn set_ecl(&mut self, ecl: String) {
            self.ecl = ecl;
        }
        fn set_cid(&mut self, cid: String) {
            self.cid = cid;
        }
        fn set_pid(&mut self, pid: String) {
            self.pid = pid;
        }
        pub fn is_valid(&self) -> bool {
            //Only cid can be blank
            self.byr.trim().len() > 0
                && self.iyr.trim().len() > 0
                && self.eyr.trim().len() > 0
                && self.hgt.trim().len() > 0
                && self.hcl.trim().len() > 0
                && self.ecl.trim().len() > 0
                && self.pid.trim().len() > 0
        }
    }
    impl fmt::Display for Passport {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "(byr:{}, iyr:{}, eyr:{}, hgt:{}, hcl:{}, ecl:{}, cid:{}, pid:{})",
                self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.cid, self.pid
            )
        }
    }
    pub fn read_strings_as_vector(filename: impl AsRef<Path>) -> Vec<Passport> {
        let file = File::open(filename).expect("Could not read");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .map(|l| l.parse::<String>().unwrap())
            .collect();
        let mut passports: Vec<Passport> = Vec::new();
        let mut passport: Passport = Passport::new();
        for line in &lines {
            if line.trim().len() == 0 {
                println!("Passport - {}", passport);
                passports.push(passport);
                passport = Passport::new();
            } else {
                let field_split: Vec<&str> = line.split(" ").collect();
                println!("field_split - {}", line);
                for field in field_split {
                    let field_value_split: Vec<&str> = field.trim().split(":").collect();
                    if field_value_split[0].trim().eq("byr") {
                        passport.set_byr(String::from(field_value_split[1]));
                    } else if field_value_split[0].trim().eq("iyr") {
                        passport.set_iyr(String::from(field_value_split[1]));
                    } else if field_value_split[0].trim().eq("eyr") {
                        passport.set_eyr(String::from(field_value_split[1]));
                    } else if field_value_split[0].trim().eq("hgt") {
                        passport.set_hgt(String::from(field_value_split[1]));
                    } else if field_value_split[0].trim().eq("hcl") {
                        passport.set_hcl(String::from(field_value_split[1]));
                    } else if field_value_split[0].trim().eq("ecl") {
                        passport.set_ecl(String::from(field_value_split[1]));
                    } else if field_value_split[0].trim().eq("pid") {
                        passport.set_pid(String::from(field_value_split[1]));
                    } else if field_value_split[0].trim().eq("cid") {
                        passport.set_cid(String::from(field_value_split[1]));
                    }
                }
            }
        }
        passports
    }
}
