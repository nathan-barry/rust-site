use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn Header(cx: Scope) -> Element {
    render! {
        div {
            class: "md:p-16 p-8 mt-16 max-w-[1000px] mx-auto",

            h1 { class: "mb-16 hover:text-highlight",
                Link { to:Route::Home {},
                    "nathan.rs" }
                }

            Outlet::<Route> {}
        }
    }
}
