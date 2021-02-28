use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Yo,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
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