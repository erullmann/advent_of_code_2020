use std::error::Error;
use regex::Regex;

// # Question 3 Expense Report
// Taking the test input, find the number of valid passwords.

pub fn answer() -> super::Answer {
    let file = super::get_file("data/day_2/input.txt");
    match file {
        Err(_) => super::Answer{ result: file, question: 3 },
        Ok(contents) => super::Answer{ result: validate_passwords(contents), question: 3 }
    }
}

#[derive(Debug)]
pub struct PasswordRecord {
    pub letter: Option<char>,
    pub min_count: Option<usize>,
    pub max_count: Option<usize>,
    pub password: Option<String>
}

pub fn parse_password_file(password_file: String) -> Result<Vec<PasswordRecord>, Box<dyn Error>> {
    let mut results = Vec::new();

    for line in password_file.lines() {
        let password_line_regex = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)");
        let captures = password_line_regex?.captures(line);

        if captures.is_none() {
            return Err(Box::new(super::AnswerError::new("Could not use regex")));
        }
        let captures = captures.unwrap();
        
        results.push(PasswordRecord {
            letter: captures.get(3).map_or(None, |m| m.as_str().to_string().chars().next()),
            min_count: captures.get(1).map_or(None, |m| m.as_str().parse::<usize>().ok()),
            max_count: captures.get(2).map_or(None, |m| m.as_str().parse::<usize>().ok()),
            password:captures.get(4).map_or(None, |m| Some(m.as_str().to_string())),
        })

    }
    Ok(results)
}

fn validate_password_record(password_record: PasswordRecord) -> bool {
    if password_record.letter.is_none() || 
        password_record.min_count.is_none() ||
        password_record.max_count.is_none() ||
        password_record.password.is_none() {
        false
    }
    else {
        let password = password_record.password.unwrap();
        let count_range = password_record.min_count.unwrap()..(password_record.max_count.unwrap() + 1);
        count_range.contains(&password.matches(password_record.letter.unwrap()).count())
    }

}

fn validate_passwords(password_file: String) -> Result<String, Box<dyn Error>> {
    let password_records = parse_password_file(password_file);
    let mut number_valid = 0;
    for record in password_records? {
        if validate_password_record(record){
            number_valid += 1;
        }
    }
    Ok(number_valid.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_total(){
        let contents = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc".to_string();

        assert_eq!("2".to_string(), validate_passwords(contents).unwrap())
    }
}