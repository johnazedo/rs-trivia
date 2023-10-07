use std::io;

static URL: &str = "https://opentdb.com/api.php?amount=10";
const NUMBER_OF_QUESTIONS: usize = 1;

fn main() {
    let questions = get_questions();
    for q in questions {
        println!("{}", format_question(&q));
        let selected = read_user_answer();
        if !check_if_answer_is_valid(&selected) {
            println!("{}", "Invalid answer")
        }
    }
}

// Change this to get a error or i32
fn read_user_answer() -> i32 {
    const READ_PROBLEM: &str = "problem to read value";

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect(READ_PROBLEM);
    return user_input.trim().parse::<i32>().expect(READ_PROBLEM)
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