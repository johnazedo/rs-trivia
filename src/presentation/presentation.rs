static URL: &str = "https://opentdb.com/api.php?amount=10";
const NUMBER_OF_QUESTIONS: usize = 1;

struct Presentation {}

impl Presentation {
    fn read_user_answer(&self) -> Result<i32, ParseIntError> {
        const READ_PROBLEM: &str = "problem to read value";
    
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect(READ_PROBLEM);
        return user_input.trim().parse::<i32>()
    }

    fn check_if_answer_is_valid(aws: &i32) -> bool {
        return *aws <= 4 && *aws > 0;
    } 

    fn format_question(&self, q: &State) -> String {
        format!("{}\n1 - {}\n2 - {}\n3 - {}\n4 - {}", 
            q.question, 
            q.answer[0],
            q.answer[1],
            q.answer[2],
            q.answer[3]
        )
    }
}


impl Subscriber for Presentation {
    fn update(&self, state: &observer::State) {
        println!("This presentation was updated");
        println!("{}", state.question)
    }
}
