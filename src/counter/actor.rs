use crate::{actor::Actor, counter::message::CounterMessage};


pub(super) struct CounterActor {
    count: u32,
}

impl CounterActor {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    fn get_count(&self) -> u32 {
        self.count
    }

    fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    fn reset_count(&mut self) {
        self.count = 0;
    }
}

impl Actor for CounterActor {
    type Message = CounterMessage;

    fn handle_message(&mut self, msg: Self::Message) {
        match msg {
            CounterMessage::GetCount { respond_to } => {
                let count = self.get_count();
                let _ = respond_to.send(count);
            }
            CounterMessage::SetCount { count } => {
                self.set_count(count);
            }
            CounterMessage::ResetCount => {
                self.reset_count();
            }
        }
    }
}
