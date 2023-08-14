#![allow(non_snake_case)]

mod projects;
mod pages;
mod components;

use crate::projects::game_of_life::GameOfLife;
use crate::pages::{
    home::Home,
    books::Books,
    page_not_found::PageNotFound
};
use crate::components::header::Header;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    #[layout(Header)]
        #[route("/")]
        Home {},
        #[route("/game")]
        GameOfLife {},
        #[route("/books")]
        Books {},
    #[end_layout]
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

