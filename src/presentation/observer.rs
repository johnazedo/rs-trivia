trait Subscriber { 
    fn update(&self, state: &State);
}

pub struct Publisher { 
    state: State,
    subscribers: Vec<Box<dyn Subscriber>>
}

pub struct State {
    question: String,
    answer: [String;4]
}

impl Publisher {
    pub fn subscribe(&mut self, s: Box<dyn Subscriber>) {
        self.subscribers.push(s);
    }

    fn set_state(&mut self, s: State) {
        self.state = s;
        self.notify_subscribers()
    }

    fn notify_subscribers(&self) {
        for s in self.subscribers.iter() {
            s.update(&self.state)
        }
    }
}