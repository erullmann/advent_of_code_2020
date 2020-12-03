use std::error::Error;

pub fn answer() -> super::Answer {
    Answer{ result = Ok(String::from("This is the answer to 1")), question = 1 }
}