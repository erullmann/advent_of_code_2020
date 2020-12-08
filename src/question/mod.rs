use std::error::Error;
use std::fmt;
use std::fs;

pub mod question_1;
pub mod question_2;
pub mod question_3;
pub mod question_4;
pub mod question_5;
pub mod question_6;

pub struct Answer {
    pub result: Result<String, Box<dyn Error>>,
    pub question: usize,
}

#[derive(Debug, PartialEq)]
pub struct AnswerError(pub String);

impl AnswerError {
    fn new(s: &str) -> AnswerError {
        AnswerError(s.to_string())
    }
}

impl fmt::Display for AnswerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for AnswerError {}

fn get_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(file_name)?)
}