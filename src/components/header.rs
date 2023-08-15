use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn Header(cx: Scope) -> Element {
    render! {
        div {
            class: "p-4 mt-12 md:p-16 md:mt-16 max-w-[1050px] mx-auto",

            h1 { class: "text-center italic mb-12 md:mb-16 hover:text-highlight",
                Link { to:Route::Home {},
                    "nathan.rs" }
                }

            Outlet::<Route> {}
        }
    }
}
