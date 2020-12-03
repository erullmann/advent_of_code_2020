use std::error::Error;
use std::fmt;

pub mod question_1;

pub struct Answer {
    pub result: Result<String, Box<AnswerError>>,
    pub question: usize,
}

#[derive(Debug)]
pub struct AnswerError(pub String);

impl fmt::Display for AnswerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for AnswerError {}