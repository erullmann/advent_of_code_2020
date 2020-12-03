use std::error::Error;
use question::Answer;

mod question;


pub struct Config {
    pub question: Option<usize>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        let question = if args.len() < 2 {
            None
        }
        else {
            Some(args[1].clone().parse::<usize>().unwrap_or_default())
        };
        
        Ok(Config { question })
    }
}

pub fn run(config: Config) {
    let question_number = config.question;

    let answers = match config.question {
        None => answer_all(),
        Some(i) => {
            let mut result = Vec::new();
            result.push(answer_question(i));
            result
        },
    };
    for (index, answer) in answers.iter().enumerate() {
        match answer.result {
            Ok(x) => {
                println!("The answer to question {} is...", index);
                println!("{}", x)
            },
            Err(e) => {
                println!("Application error: {}", e);
            }
        };
    }
}

fn answer_all() -> Vec<question::Answer> {
    let mut results = Vec::new();
    for i in 1..2 {
        results.push(answer_question(i));
    }
    results
}

fn answer_question(question_number: usize) -> Answer {
    match question_number {
        1 => question::question_1::answer(),
        _ => Answer { result: Err(format!("Implementation not found for {}", question_number)), question: 0 }
    }
}
