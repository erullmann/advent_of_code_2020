use std::fs;
use std::error::Error;
use std::collections::HashMap;

// # Question 1 Expense Report
// Taking the test input, find the two numbers that add up to 2020 and return their product.

const REQUIRED_SUM: usize = 2020;

pub fn answer() -> super::Answer {
    let file = get_file();
    match file {
        Err(_) => super::Answer{ result: file, question: 1 },
        Ok(contents) => super::Answer{ result: expense_report(contents), question: 1 }
    }
}

fn get_file() -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string("data/day_1/input.txt")?)
}

fn expense_report(contents: String) -> Result<String, Box<dyn Error>> {
    let mut required_numbers : HashMap<usize, bool> = HashMap::new();

    for line in contents.lines() {
        let num: usize = line.parse::<usize>()?;
        let inverse_num = REQUIRED_SUM - num;

        if required_numbers.contains_key(&inverse_num) {
            return Ok((num * inverse_num).to_string())
        }
        required_numbers.insert(num, true);
    }
    Err(Box::new(super::AnswerError(format!("Numbers summing to {} could not be found.", REQUIRED_SUM))))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_total(){
        let contents = "\
1721
979
366
299
675
1456".to_string();

        assert_eq!("514579".to_string(), expense_report(contents).unwrap())
    }
}