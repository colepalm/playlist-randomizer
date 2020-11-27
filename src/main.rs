#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/randomize")]
fn randomize() -> &'static str {

}

fn main() {
    rocket::ignite().mount("/", routes![index, randomize]).launch();
}