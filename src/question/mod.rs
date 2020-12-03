pub mod question_1;

pub struct Answer {
    pub Result<String, Box<dyn Error>> result;
    pub usize question;
}