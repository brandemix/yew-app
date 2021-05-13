use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

pub enum Request {
    Increment,
    Decrement,
}

pub enum Action {
    SetCount(i64),
}

pub struct CountStore {
    pub count: i64,
}

impl Store for CountStore {
    type Action = Action;
    type Input = Request;

    fn new() -> Self {
        CountStore { count: 0 }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Request::Increment => {
                let tmp = self.count + 1;
                link.send_message(Action::SetCount(tmp));
            }
            Request::Decrement => {
                let tmp = self.count - 1;
                link.send_message(Action::SetCount(tmp));
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetCount(count) => self.count = count,
        }
    }
}
