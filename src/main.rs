use std::{io, num::ParseIntError};

static URL: &str = "https://opentdb.com/api.php?amount=10";
const NUMBER_OF_QUESTIONS: usize = 1;

fn main() {
    // let questions = get_questions();
    // for q in questions {
    //     loop {
    //         println!("{}", format_question(&q));
    //         let choice = read_user_answer();
    //         match choice {
    //             Err(_error) => {
    //                 println!("This anwser is not valid. Try again.\n");
    //                 continue;
    //             },
    //             Ok(value) => {
    //                 if !check_if_answer_is_valid(&value) {
    //                     println!("Invalid answer\n");
    //                     continue;
    //                 }
    //             }
    //         }
    //     }
    // }
    


}

fn read_user_answer() -> Result<i32, ParseIntError> {
    const READ_PROBLEM: &str = "problem to read value";

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect(READ_PROBLEM);
    return user_input.trim().parse::<i32>()
}

fn check_if_answer_is_valid(aws: &i32) -> bool {
    return *aws <= 4 && *aws > 0;
} 

fn format_question(q: &Question) -> String {
    format!("{}\n1 - {}\n2 - {}\n3 - {}\n4 - {}", 
        q.question, 
        q.correct_answer,
        q.incorrect_answers[0],
        q.incorrect_answers[1],
        q.incorrect_answers[2]
    )
}

struct Question {
    category: String,
    question_type: String,
    difficult: String,
    question: String,
    correct_answer: String,
    incorrect_answers: [String;3]
}

fn get_questions() -> [Question; NUMBER_OF_QUESTIONS] {
    return [
        Question {
            category: String::from("Science: Computers"),
            question_type: String::from("multiple"),
            difficult: String::from("medium"),
            question: String::from("Which internet company began life as an online bookstore called &#039;Cadabra&#039;?"),
            correct_answer: String::from("Amazon"),
            incorrect_answers: [
                String::from("eBay"),
                String::from("Overstock"),
                String::from("Shopify")
            ]
        }
    ]
}

struct Presentation {}

trait Subscriber { 
    fn update(&self, state: State);
}

impl Subscriber for Presentation {
    fn update(&self, state: State) {
        println!("This presentation was updated")
    }
}

struct Publisher { 
    state: State,
    subscribers: Vec<Box<dyn Subscriber>>
}

struct State {
    question: String,
    answer: [String;4]
}

impl Publisher {
    fn subscribe(&mut self, s: Box<dyn Subscriber>) {
        self.subscribers.push(s);
    }

    fn set_state(&mut self, s: State) {
        self.state = s;
        self.notify_subscribers()
    }

    fn notify_subscribers(&self) {
        for s in self.subscribers.iter() {
            s.update(self.state)
        }
    }
}