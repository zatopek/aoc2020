extern crate regex;

pub mod input_readers {
    use regex::Regex;
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
        pub fn new() -> Passport {
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
        pub fn set_byr(&mut self, byr: String) {
            self.byr = byr;
        }
        pub fn set_iyr(&mut self, iyr: String) {
            self.iyr = iyr;
        }
        pub fn set_eyr(&mut self, eyr: String) {
            self.eyr = eyr;
        }
        pub fn set_hgt(&mut self, hgt: String) {
            self.hgt = hgt;
        }
        pub fn set_hcl(&mut self, hcl: String) {
            self.hcl = hcl;
        }
        pub fn set_ecl(&mut self, ecl: String) {
            self.ecl = ecl;
        }
        pub fn set_cid(&mut self, cid: String) {
            self.cid = cid;
        }
        pub fn set_pid(&mut self, pid: String) {
            self.pid = pid;
        }
        pub fn is_byr_valid(&self) -> bool {
            if !self.byr.trim().is_empty() {
                let byr_re:Regex = Regex::new(r"^19[2-9]{1}[0-9]{1}|^200[0-2]{1}$").unwrap();
                return byr_re.is_match(self.byr.trim());
            }
            false
        }
        pub fn is_iyr_valid(&self) -> bool {
            if !self.iyr.trim().is_empty() {
                let iyr_re: Regex = Regex::new(r"^(201[0-9]{1}|2020)$").unwrap();
                return iyr_re.is_match(self.iyr.trim());
            }
            false
        }
        pub fn is_eyr_valid(&self) -> bool {
            if !self.eyr.trim().is_empty() {
                let eyr_re: Regex = Regex::new(r"^(202[0-9]{1}|2030)$").unwrap();
                return eyr_re.is_match(self.eyr.trim());
            }
            false
        }
        pub fn is_hgt_valid(&self) -> bool {
            if !self.hgt.trim().is_empty() {
                let hgt_re: Regex = Regex::new(r"^((59|6[0-9]|7[0-6])in|(1[5-8][0-9]|19[0-3])cm)$").unwrap();
                return hgt_re.is_match(self.hgt.trim());
            }
            false
        }
        pub fn is_hcl_valid(&self) -> bool {
            if !self.hcl.trim().is_empty() {
                let hcl_re: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                return hcl_re.is_match(self.hcl.trim());
            }
            false
        }
        pub fn is_ecl_valid(&self) -> bool {
            if !self.ecl.trim().is_empty() {
                let ecl_re: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                return ecl_re.is_match(self.ecl.trim());
            }
            false
        }
        pub fn is_pid_valid(&self) -> bool {
            if !self.pid.trim().is_empty() {
                let pid_re: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
                return pid_re.is_match(self.pid.trim());
            }
            false
        }

        pub fn is_valid(&self) -> bool {
            //Only cid can be blank
            self.is_byr_valid()
                && self.is_iyr_valid()
                && self.is_eyr_valid()
                && self.is_hgt_valid()
                && self.is_hcl_valid()
                && self.is_ecl_valid()
                && self.is_pid_valid()
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
                passports.push(passport);
                passport = Passport::new();
            } else {
                let field_split: Vec<&str> = line.split(" ").collect();
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

#[cfg(test)]
mod tests {

    use super::input_readers::Passport;

    #[test]
    fn check_byr() {
        let mut passport: Passport = Passport::new();
        passport.set_byr(String::from("1920"));
        assert_eq!(passport.is_byr_valid(), true);
        passport.set_byr(String::from("2002"));
        assert_eq!(passport.is_byr_valid(), true);
        passport.set_byr(String::from(""));
        assert_eq!(passport.is_byr_valid(), false);
        passport.set_byr(String::from("1983"));
        assert_eq!(passport.is_byr_valid(), true);
        passport.set_byr(String::from("2003"));
        assert_eq!(passport.is_byr_valid(), false);
        passport.set_byr(String::from("203"));
        assert_eq!(passport.is_byr_valid(), false);
        passport.set_byr(String::from("2203"));
        assert_eq!(passport.is_byr_valid(), false);
        passport.set_byr(String::from("20321"));
        assert_eq!(passport.is_byr_valid(), false);
    }

    #[test]
    fn check_iyr() {
        let mut passport: Passport = Passport::new();
        passport.set_iyr(String::from("2010"));
        assert_eq!(passport.is_iyr_valid(), true);
        passport.set_iyr(String::from("2020"));
        assert_eq!(passport.is_iyr_valid(), true);
        passport.set_iyr(String::from(""));
        assert_eq!(passport.is_iyr_valid(), false);
        passport.set_iyr(String::from("2009"));
        assert_eq!(passport.is_iyr_valid(), false);
        passport.set_iyr(String::from("02012"));
        assert_eq!(passport.is_iyr_valid(), false);
        passport.set_iyr(String::from("203"));
        assert_eq!(passport.is_iyr_valid(), false);
        passport.set_iyr(String::from("2013"));
        assert_eq!(passport.is_iyr_valid(), true);
        passport.set_iyr(String::from("20321"));
        assert_eq!(passport.is_iyr_valid(), false);
    }

    #[test]
    fn check_eyr() {
        let mut passport: Passport = Passport::new();
        passport.set_eyr(String::from("2020"));
        assert_eq!(passport.is_eyr_valid(), true);
        passport.set_eyr(String::from("2030"));
        assert_eq!(passport.is_eyr_valid(), true);
        passport.set_eyr(String::from(""));
        assert_eq!(passport.is_eyr_valid(), false);
        passport.set_eyr(String::from("1983"));
        assert_eq!(passport.is_eyr_valid(), false);
        passport.set_eyr(String::from("2023"));
        assert_eq!(passport.is_eyr_valid(), true);
        passport.set_eyr(String::from("02023"));
        assert_eq!(passport.is_eyr_valid(), false);
        passport.set_eyr(String::from("2203"));
        assert_eq!(passport.is_eyr_valid(), false);
        passport.set_eyr(String::from("20321"));
        assert_eq!(passport.is_eyr_valid(), false);
    }
    #[test]
    fn check_hgt() {
        let mut passport: Passport = Passport::new();
        passport.set_hgt(String::from("167in"));
        assert_eq!(passport.is_hgt_valid(), false);
    }

    #[test]
    fn check_hcl() {
        let mut passport: Passport = Passport::new();
        passport.set_hcl(String::from("#282626"));
        assert_eq!(passport.is_hcl_valid(), true);
        passport.set_hcl(String::from("#282a26"));
        assert_eq!(passport.is_hcl_valid(), true);
        passport.set_hcl(String::from("#f82626"));
        assert_eq!(passport.is_hcl_valid(), true);
        passport.set_hcl(String::from("#fffaaa"));
        assert_eq!(passport.is_hcl_valid(), true);
        passport.set_hcl(String::from("#d282626"));
        assert_eq!(passport.is_hcl_valid(), false);
        passport.set_hcl(String::from("#28262ds6"));
        assert_eq!(passport.is_hcl_valid(), false);
    }
}