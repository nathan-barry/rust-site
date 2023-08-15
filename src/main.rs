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
    creating_this_website::CreatingThisWebsite,
};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlScriptElement, HtmlLinkElement, Window};


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
        #[route("/game")]
        GameOfLife {},
        // BLOG
        // RUST
        #[route("/creating-this-website")]
        CreatingThisWebsite {},
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
    let _ = load_highlight_js().unwrap();
    render! {
        Router::<Route> {}
    }
}

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}


// This loads highlight.js for code block syntax highlighting
#[wasm_bindgen]
pub fn load_highlight_js() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No global `window` exists.")?;
    let document = window.document().ok_or("Should have a document on window.")?;
    let head = document.head().ok_or("Document should have a head.")?;

    // Create a new link element
    let link: HtmlLinkElement = document
        .create_element("link")?
        .dyn_into::<HtmlLinkElement>()?;
    link.set_rel("stylesheet");
    link.set_href("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.7.2/styles/vs2015.min.css");
    head.append_child(&link)?;

    // Create a new script element
    let script: HtmlScriptElement = document
        .create_element("script")?
        .dyn_into::<HtmlScriptElement>()?;
    script.set_src("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.7.2/highlight.min.js");
    head.append_child(&script)?;


    // Create a closure to call hljs.highlightAll() once the script has loaded
    let closure = Closure::wrap(Box::new(|| {
        let js_code = "hljs.highlightAll();";
        let _ = js_sys::eval(js_code).expect("Error highlighting code");
    }) as Box<dyn Fn()>);
    // Set the closure as the onload event for the script
    script.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    head.append_child(&script)?;

    Ok(())
}
