use question::Answer;
use question::AnswerError;

mod question;

const MAX_QUESTION: usize = 4;

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
    let answers = match config.question {
        None => answer_all(),
        Some(i) => {
            let mut result = Vec::new();
            result.push(answer_question(i));
            result
        },
    };
    for answer in answers {
        match &answer.result {
            Ok(x) => {
                println!("The answer to question {} is...", answer.question);
                println!("{}", x);
            },
            Err(e) => {
                println!("Application error: {}", e);
            }
        };
    }
}

fn answer_all() -> Vec<question::Answer> {
    let mut results = Vec::new();
    for i in 1..(MAX_QUESTION + 1) {
        results.push(answer_question(i));
    }
    results
}

fn answer_question(question_number: usize) -> Answer {
    match question_number {
        1 => question::question_1::answer(),
        2 => question::question_2::answer(),
        3 => question::question_3::answer(),
        4 => question::question_4::answer(),

        _ => Answer { result: Err(Box::new(AnswerError(format!("Implementation not found for {}", question_number)))), question: 0 }
    }
}
