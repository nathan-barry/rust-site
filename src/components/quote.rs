use dioxus::prelude::*;

#[inline_props]
pub fn Quote(
    cx: Scope,
    quote: String, 
    author: String, 
) -> Element {
    render! {
        p { class: "m-16 mb-0 text-center italic text-grey",
            "{quote}"
        }
        p { class: "m-16 mt-2 text-right italic text-lg text-grey",
            "{author}"
        }
    }
}
