// # Question 7 Password Control

use regex::Regex;

pub fn answer() -> super::Answer {
    let file = super::get_file("data/day_4/input.txt");
    match file {
        Err(_) => super::Answer{ result: file, question: 7 },
        Ok(contents) => {
            let passport_count = count_correct_passports(&contents);
            super::Answer{ result: Ok(passport_count.to_string()), question: 7 }
        }
    }
}


#[derive(Debug)]
pub struct Passport {
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub eyr: Option<String>,
    pub hcl: Option<String>,
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub cid: Option<String>,
    pub hgt: Option<String>
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            ecl: None,
            pid: None,
            eyr: None,
            hcl: None,
            byr: None,
            iyr: None,
            cid: None,
            hgt: None
        }
    }
    
    pub fn valid(&self) -> bool {
        return self.ecl.is_some() &&
            self.pid.is_some() &&
            self.eyr.is_some() &&
            self.hcl.is_some() &&
            self.byr.is_some() &&
            self.iyr.is_some() &&
            self.hgt.is_some()
    }
    
    pub fn add_fields(&mut self, line: &str) {
        let passport_line_regex = Regex::new(r"([a-z]{3}):([a-z0-9#]+)").unwrap();
        for capture in passport_line_regex.captures_iter(line) {
            let field = capture.get(1).unwrap().as_str();
            let value = capture.get(2).unwrap().as_str().to_string();
            println!("added field {}, value {}", field, value);
            
            match field {
                "ecl" => self.ecl = Some(value),
                "pid" => self.pid = Some(value),
                "eyr" => self.eyr = Some(value),
                "hcl" => self.hcl = Some(value),
                "byr" => self.byr = Some(value),
                "iyr" => self.iyr = Some(value),
                "cid" => self.cid = Some(value),
                "hgt" => self.hgt = Some(value),
                &_ => println!("uknown field: {}, value: {}", field, value)
            }
        }
    }
}

pub fn count_correct_passports(contents: &String) -> usize {
    let mut passport_count = 0;
    let mut current_passport = Passport::new();

    for line in contents.lines() {
        if line.is_empty() {
            if current_passport.valid() {
                passport_count += 1;
            }
            current_passport = Passport::new();
        }
        else {
            current_passport.add_fields(line);
        }
    }
    if current_passport.valid() {
        passport_count += 1;
    }
    return passport_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_passports(){
        let passports: String = 
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in".to_string();
        assert_eq!(2, count_correct_passports(&passports));
    }
}