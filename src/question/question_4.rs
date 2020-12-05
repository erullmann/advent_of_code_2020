use std::error::Error;
use super::question_3::PasswordRecord;
use super::question_3::parse_password_file;


pub fn answer() -> super::Answer {
    let file = super::get_file("data/day_2/input.txt");
    match file {
        Err(_) => super::Answer{ result: file, question: 4 },
        Ok(contents) => super::Answer{ result: validate_passwords(contents), question: 4 }
    }
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
        let first_index = password_record.min_count.unwrap() - 1;
        let second_index = password_record.max_count.unwrap() - 1;

        let first_char = password.chars().nth(first_index).unwrap();
        let second_char = password.chars().nth(second_index).unwrap();
        (first_char == password_record.letter.unwrap()) ^ (second_char == password_record.letter.unwrap())
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

        assert_eq!("1".to_string(), validate_passwords(contents).unwrap())
    }
}