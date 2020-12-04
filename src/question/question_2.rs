use std::fs;
use std::error::Error;
use std::collections::HashMap;

// # Question 2 Expense Report
// Taking the test input, find the three numbers that add up to 2020 and return their product.

const REQUIRED_SUM: usize = 2020;

pub fn answer() -> super::Answer {
    let file = get_file();
    match file {
        Err(_) => super::Answer{ result: file, question: 2 },
        Ok(contents) => super::Answer{ result: expense_report(contents), question: 2 }
    }
}

fn get_file() -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string("data/day_1/input.txt")?)
}

fn expense_report(contents: String) -> Result<String, Box<dyn Error>> {
    // the sum of every pair of numbers, along with the numbers
    let mut two_number_sums : HashMap<usize, (usize, usize)> = HashMap::new();
    let mut converted_numbers :Vec<usize> = Vec::new();

    for line in contents.lines() {
        converted_numbers.push(line.parse()?)
    }

    for num1 in &converted_numbers {
        for num2 in &converted_numbers {
            two_number_sums.insert(num1 + num2, (*num1, *num2));
        }
    }

    for num in converted_numbers {
        let inverse_num = REQUIRED_SUM - num;

        if two_number_sums.contains_key(&inverse_num) {
            let other_numbers = two_number_sums[&inverse_num];
            return Ok((num * other_numbers.0 * other_numbers.1).to_string())
        }
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

        assert_eq!("241861950".to_string(), expense_report(contents).unwrap())
    }
}