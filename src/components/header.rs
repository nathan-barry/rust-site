use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn Header(cx: Scope) -> Element {
    render! {
        div {
            class: "m-16",

            h1 { class: "mb-4 text-highlight",
                "nathan.rs" }
            Outlet::<Route> {}
        }
    }
}
