use crate::components::login::Login;
use yew::prelude::*;
use std::env;

const CLIENT_ID: &'static str = "6e53103644d24100984110dbc4d61bdf";
const REDIRECT_URI: &'static str = "https://localhost:8080/callback";
const SCOPES: &'static str = "user-read-private user-read-email";

pub struct App {
    link: ComponentLink<Self>,
    client_secret: String
}

pub enum Msg {
    Login,
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

        App { link, client_secret }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login => {
                // TODO: Add redirection logic
                // redirect("https://accounts.spotify.com/authorize?"
                // querystring.stringify({
                //     response_type: 'code',
                //     client_id: client_id,
                //     scope: scope,
                //     redirect_uri: redirect_uri,
                //     state: state
                // }));
                // );
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