use crate::{content, generator::Generated};
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct Login {
    login: content::Login,
}
impl component for Login {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            login: content::Login::generate_from_seed(props.seed),
        }
    }

    fn view(&self) -> Html {
        let self { Login } = self;

        html! {
            <div>{ "login!" }</div>
        }
    }
}