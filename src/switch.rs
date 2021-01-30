use yew::{
    virtual_dom::{Transformer, VComp},
    web_sys::Url,
};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

#[derive(Switch)]
enum AppRoute {
    #[to="/login"]
    Login
}