use yew::prelude::*;
use std::env;

pub struct App {
    link: ComponentLink<Self>,
    client_id: &'static str,
    client_secret: String,
    redirect_uri: &'static str,
    scopes: &'static str
}

pub enum Msg {
    Yo,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let client_secret;
        match env::var("CLIENT_SECRET") {
            Ok(val) => client_secret = val,
            Err(_e) => client_secret = "none".to_string(),
        }

        App {
            link,
            client_id: "6e53103644d24100984110dbc4d61bdf",
            client_secret,
            redirect_uri: "https://localhost:8080/callback",
            scopes: "user-read-private user-read-email"
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <div>
                <div>
                    <a href="/login"> {"First, login to Spotify"} </a>
                </div>
            </div>
        }
    }
}