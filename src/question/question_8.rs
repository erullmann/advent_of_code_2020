// # Question 8 Password Control

use regex::Regex;
use super::question_7::Passport;

pub fn answer() -> super::Answer {
    let file = super::get_file("data/day_4/input.txt");
    match file {
        Err(_) => super::Answer{ result: file, question: 8 },
        Ok(contents) => {
            let passport_count = count_valid_and_correct_passports(&contents);
            super::Answer{ result: Ok(passport_count.to_string()), question: 8 }
        }
    }
}

impl Passport {
    pub fn valid_and_correct(&self) -> bool {
        if !self,valid() {
            return false;
        }
        
        return true;
    }
}


fn count_valid_and_correct_passports(contents: &String) -> usize {
    let mut passport_count = 0;
    let mut current_passport = Passport::new();

    for line in contents.lines() {
        if line.is_empty() {
            if current_passport.valid_and_correct() {
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
        assert_eq!(2, count_valid_and_correct_passports(&passports));
    }
}
