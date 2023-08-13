#![allow(non_snake_case)]

mod projects;

use crate::projects::game_of_life::GameOfLife;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/a")]
    Test {},
    #[route("/game")]
    GameOfLife {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Welcome!" },
        p {
            Link { to:Route::GameOfLife {},
                "Game"
            }
        },
        p { Link { to:Route::Test {},
                "Test"
            }
        },
    }
}

#[inline_props]
fn Test(cx: Scope) -> Element {
    render! {
        h1 { "Test!" }
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
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

