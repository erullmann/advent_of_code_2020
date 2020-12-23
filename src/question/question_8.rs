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
        if !self.valid() {
            return false;
        }
        
        return valid_year(&self.byr, 1920, 2002) &&
            valid_year(&self.iyr, 2010, 2020) &&
            valid_year(&self.eyr, 2020, 2030) &&
            valid_height(&self.hgt) &&
            valid_eye_color(&self.ecl) &&
            valid_hair_color(&self.hcl) &&
            valid_pid(&self.pid);
    }

}

fn valid_pid(eye: &Option<String>) -> bool {
    if eye.is_none() {
        return false;
    }
    let eye_regex = Regex::new(r"^([0-9]{9})$").unwrap();
    let eye = eye.as_ref().unwrap().clone();
    let eye_results = eye_regex.captures(&eye);

    return match eye_results {
        Some(_) => true,
        None => false
    }
}

fn valid_eye_color(eye: &Option<String>) -> bool {
    if eye.is_none() {
        return false;
    }
    let eye_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let eye = eye.as_ref().unwrap().clone();
    let eye_results = eye_regex.captures(&eye);

    return match eye_results {
        Some(_) => true,
        None => false
    }
}

fn valid_hair_color(hair: &Option<String>) -> bool {
    if hair.is_none() {
        return false;
    }
    let hair_regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    let hair = hair.as_ref().unwrap().clone();
    let hair_results = hair_regex.captures(&hair);

    return match hair_results {
        Some(_) => true,
        None => false
    }
}

fn valid_height(height: &Option<String>) -> bool {
    if height.is_none() {
        return false;
    }
    let height_regex = Regex::new(r"^([0-9]+)(in|cm)$").unwrap();
    let height = height.as_ref().unwrap().clone();
    let height_results = height_regex.captures(&height);

    return match height_results {
        Some(capture) => {
            let height_value = capture.get(1).unwrap().as_str();
            let height_unit = capture.get(2).unwrap().as_str();
            let height_value = height_value.to_string().parse::<usize>().unwrap();
            match height_unit {
                "in" => (59..=76).contains(&height_value),
                "cm" => (150..=193).contains(&height_value),
                _ => false
            }
        },
        None => false
    }
}

fn valid_year(year: &Option<String>, min_year: usize, max_year: usize) -> bool {
    if year.is_none() {
        return false;
    }
    let year = year.as_ref().unwrap().parse::<usize>();
    return year.map_or(false, |y| (min_year..=max_year).contains(&y));
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
    if current_passport.valid_and_correct() {
        passport_count += 1;
    }
    return passport_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_colors(){
        assert_eq!(true, valid_hair_color(&Some("#000000".to_string())));
        assert_eq!(true, valid_hair_color(&Some("#aabbff".to_string())));
        assert_eq!(false, valid_hair_color(&Some("#967".to_string())));
        assert_eq!(false, valid_hair_color(&Some("#000KUI".to_string())));
        assert_eq!(false, valid_hair_color(&None));

        assert_eq!(true, valid_eye_color(&Some("grn".to_string())));
        assert_eq!(true, valid_eye_color(&Some("oth".to_string())));
        assert_eq!(false, valid_eye_color(&Some("grnoth".to_string())));
        assert_eq!(false, valid_eye_color(&None));
    }


    #[test]
    fn validate_height(){
        assert_eq!(true, valid_height(&Some("180cm".to_string())));
        assert_eq!(true, valid_height(&Some("60in".to_string())));
        assert_eq!(true, valid_height(&Some("193cm".to_string())));
        assert_eq!(true, valid_height(&Some("59in".to_string())));
        assert_eq!(false, valid_height(&Some("194cm".to_string())));
        assert_eq!(false, valid_height(&Some("56in".to_string())));
        assert_eq!(false, valid_height(&Some("test".to_string())));
        assert_eq!(false, valid_height(&Some("180".to_string())));
        assert_eq!(false, valid_height(&Some("cm".to_string())));
        assert_eq!(false, valid_height(&None));
    }

    #[test]
    fn validate_years(){
        assert_eq!(true, valid_year(&Some("2020".to_string()), 2020, 2030));
        assert_eq!(true, valid_year(&Some("2030".to_string()), 2020, 2030));
        assert_eq!(false, valid_year(&Some("1920".to_string()), 2020, 2030));
        assert_eq!(false, valid_year(&Some("test".to_string()), 2020, 2030));
        assert_eq!(false, valid_year(&None, 2020, 2030));
    }

    #[test]
    fn validate_passports(){
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

    #[test]
    fn correct_passports(){
        let passports:String = 
"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string();
        assert_eq!(4, count_valid_and_correct_passports(&passports));

    }
    
    #[test]
    fn wrong_passports() {
        let passports: String = 
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007".to_string();
        assert_eq!(0, count_valid_and_correct_passports(&passports));
    }
}
