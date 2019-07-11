#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

// mod components;
// mod services;
// mod util;

// use self::{
// components::{controls::Controls, messages::Messages, stats::Stats},
// services::{websocket};
// util::*,
// };

use yew::prelude::*;

// reference/chat.js

#[derive(Debug, Clone)]
pub struct Model {
  open: bool,
  user_id: String,
}

pub enum Msg {
    Exit,
    Clear,
    Submit,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            open: false,
            user_id: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // match msg {
        //     Msg::Submit => unimplemented!(),
        //     Msg::Exit => unimplemented!(),
        //     Msg::Clear => unimplemented!(),
        //     _ => unreachable!(),
        // }
        true
    }
}

// Refer to HTML file at https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section>
                <nav id="nav", class=("nav", "flex", "center"), >
                    <a
                        class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "right-auto"),
                        href="https://www.steadylearner.com/blog",
                        title="Click it to learn how to code this.",
                    >
                        <span class=("white", "bold"), >{ "© Rust Chat App" }</span>
                    </a>
                    <span
                        id="exit",
                        class=("margin-right-one", "white", "cursor-pointer", "hover", "transition"),
                    >
                       { "Exit" }
                    </span>
                </nav>
                <ul
                    id="messages",
                >
                </ul>
                <form
                    id="form",
                    class=("chat-input", "flex", "center"),
                >
                    <img
                        id="code",
                        class=("flex", "center", "rust-icon", "hover", "cursor-pointer", "transition-half"),
                        title="Use this for whatever you want",
                        src="Rust.svg",
                    />
                    <input
                        id="msg",
                        type="text",
                        placeholder="Type here to start to talk with others.",
                        autocomplete="off",
                    />
                    <button class=("blue", "cursor-pointer"), >{ "Send" }</button>
                    <button
                        id="clear",
                        class=("margin-left-one", "red-white", "cursor-pointer"),
                        type="button",
                    >
                        { "Clear" }
                    </button>
                </form>
            </section>
        }
    }
}
