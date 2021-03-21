use crate::components::button::Button;
use yew::prelude::*;
use std::env;

const CLIENT_ID: &'static str = "6e53103644d24100984110dbc4d61bdf";
const CLIENT_SECRET: String = String::from("");
const REDIRECT_URI: &'static str = "https://localhost:8080/callback";
const SCOPES: &'static str = "user-read-private user-read-email";

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Login,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        match env::var("CLIENT_SECRET") {
            Ok(val) => CLIENT_SECRET = val,
            Err(_e) => CLIENT_SECRET = "none".to_string(),
        }

        App { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login => {
                self.counter += 1;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <div>
                <Login onsignal=self.link.callback(|_| Msg::Login) />
            </div>
        }
    }
}