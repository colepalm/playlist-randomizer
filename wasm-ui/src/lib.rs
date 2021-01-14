#![recursion_limit="1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    value: i64,
    client_id: & 'static str,
    client_secret: & 'static str,
    redirect_uri: & 'static str
}

enum Msg { }

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
            client_id: "6e53103644d24100984110dbc4d61bdf",
            client_secret: "753618640480407284c304ffa247fbe6",
            redirect_uri: "http://localhost:8080"
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="login">
                <h1>{ "First, log in to spotify" }</h1>
                <a href="/login">{ "Log in" }</a>
            </div>
            // <div>
            //     <h1>{ "Logged in as " }</h1>
            //     <img id="avatar" width="200" src="" />
            //     <dl>
            //         <dt>{ "Display name" }</dt><dd></dd>
            //         <dt>{ "Username" }</dt><dd></dd>
            //         <dt>{ "Email" }</dt><dd></dd>
            //         <dt>{ "Spotify URI" }</dt><dd><a href=""></a></dd>
            //         <dt>{ "Link" }</dt><dd><a href=""></a></dd>
            //         <dt>{ "Profile Image" }</dt><dd></dd>
            //     </dl>
            //     <p><a href="/">{ "Log in again" }</a></p>
            // </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}