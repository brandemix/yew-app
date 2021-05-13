mod agents;

use agents::count::{CountStore, Request};
use yew::prelude::*;
use yew::services::ConsoleService;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

enum Msg {
    AddOne,
    SubtractOne,
    CountStoreMsg(ReadOnly<CountStore>),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component.
    link: ComponentLink<Self>,
    count: i64,
    count_store: Box<dyn Bridge<StoreWrapper<CountStore>>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::CountStoreMsg);
        Self {
            link,
            count: 0,
            count_store: CountStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.count_store.send(Request::Increment);
                false
            }
            Msg::SubtractOne => {
                self.count_store.send(Request::Decrement);
                false
            }
            Msg::CountStoreMsg(state) => {
                ConsoleService::log("Received Update");
                let state = state.borrow();
                if state.count != self.count {
                    self.count = state.count;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return false.
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::SubtractOne)>{ "-1" }</button>
                <p>{ self.count }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
