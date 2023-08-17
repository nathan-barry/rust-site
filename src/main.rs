#![allow(non_snake_case)]

mod projects;
mod pages;
mod components;
mod blog;

use crate::projects::game_of_life::GameOfLife;
use crate::pages::{
    home::Home,
    books::Books,
    page_not_found::PageNotFound,
    about_me::AboutMe,
};
use crate::components::wrapper::Wrapper;
use crate::blog::{
    this_mountain_we_climb::ThisMountainWeClimb,
    geb_transformers::GEBTransformers,
    summer_plan_2023::SummerPlan2023,
    experience_with_rust_front_end::ExperienceWithRustFrontEnd,
};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    // Ass but not worth the time figuring out how to not do it this way
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        // PERSONAL
        #[route("/about")]
        AboutMe {},
        #[route("/books")]
        Books {},
        // PROJECTS
        #[route("/game-of-life")]
        GameOfLife {},
        // BLOG
        // RUST
        #[route("/my-expereince-with-rust-front-end")]
        ExperienceWithRustFrontEnd {},
        // ML
        #[route("/geb-transformers")]
        GEBTransformers {},
        // LIFE
        #[route("/summer-plan-2023")]
        SummerPlan2023 {},
        // MISC
        #[route("/this-mountain-we-climb")]
        ThisMountainWeClimb {},
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
