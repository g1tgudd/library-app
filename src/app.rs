//IMPORT 
use yew::prelude::*;
use std::rc::Rc;
use yewdux::prelude::*;

use yew::{
    services::{
        ConsoleService,
    },
};

use crate::router::{
    render::Render,
    // render_guest::RenderGuest,
    // render_member::RenderMember,
};

use crate::store::store:: {
    CounterStore,
    CounterOutput,
    State,
};


pub enum Msg {
    State(Rc<State>),
    Output(CounterOutput),
}

pub struct App {
    dispatch: Dispatch<CounterStore>,
    username: Option<String>,

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dispatch = {
            let on_state = link.callback(Msg::State);
            let on_output = link.callback(Msg::Output);

            Dispatch::bridge(on_state, on_output)
        };

        Self {
            dispatch,
            username: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                ConsoleService::info("page app.rs");
                ConsoleService::info(&format!("state is {:?}", state));
                self.username= state.username.clone();
                true
            }
            
            Msg::Output(msg) => {
                match msg { //HANDLE OUTPUT
                    // CounterOutput::Doubled(n) => {
                    //     ConsoleService::info(&format!("count doubled would be {:?}", n));
                    //     true
                    // }
                    // CounterOutput::AddFive(n) => {
                    //     ConsoleService::info(&format!("count plus five would be {:?}", n));
                    //     true
                    // }
                     _ => {
                        ConsoleService::info(&format!("ignored"));
                        false
                    }
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {

        // let is_logged_in = self.username.is_some();

        // if is_logged_in {
        //     html!{
        //         <div>
        //             <RenderMember/>
        //         </div>
        //     }
        // }else {
        //     html! {
        //         <div>
        //             <RenderGuest/>
        //         </div>
        //     }
        // }

        html! {
            <div>
                <Render/>

            </div>
        }
    }
}
