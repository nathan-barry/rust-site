#![allow(non_snake_case)]

mod projects;
mod pages;
mod components;
mod blog;

use crate::projects::game_of_life::GameOfLife;
use crate::pages::{
    home::Home,
    books::Books,
    page_not_found::PageNotFound
};
use crate::components::header::Header;
use crate::blog::{
    this_mountain_we_climb::ThisMountainWeClimb,
    geb_transformers::GEBTransformers,
    test_post::TestPost,
};

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
        // Not worth the time figuring out how to not do it this way
        #[route("/this-mountain-we-climb")]
        ThisMountainWeClimb {},
        #[route("/geb-transformers")]
        GEBTransformers {},
        #[route("/test-post")]
        TestPost {},
    #[end_layout]
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

