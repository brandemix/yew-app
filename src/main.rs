mod agents;

use agents::universe::{Request as UniverseRequest, UniverseStore};
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::ConsoleService;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

enum Msg {
    RenderLoopTick,
    UniverseStoreMsg(ReadOnly<UniverseStore>),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component.
    link: ComponentLink<Self>,
    cells: String,
    universe_store: Box<dyn Bridge<StoreWrapper<UniverseStore>>>,
    _task: IntervalTask,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let render_loop = link.callback(|_| Msg::RenderLoopTick);
        let callback = link.callback(Msg::UniverseStoreMsg);
        Self {
            link,
            cells: String::from(""),
            universe_store: UniverseStore::bridge(callback),
            _task: IntervalService::spawn(Duration::from_millis(1), render_loop),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RenderLoopTick => {
                ConsoleService::log("Render Loop");
                self.universe_store.send(UniverseRequest::Tick);
                false
            }
            Msg::UniverseStoreMsg(state) => {
                let state = state.borrow();
                self.cells = state.universe.render();
                true
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
                <pre id="game-of-life-canvas">{&self.cells}</pre>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
