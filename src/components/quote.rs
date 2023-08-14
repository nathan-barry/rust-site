use dioxus::prelude::*;

#[inline_props]
pub fn Quote(
    cx: Scope,
    quote: String, 
    author: String, 
) -> Element {
    render! {
        p { class: "mx-4 mt-12 md:mx-16 md:mt-16 md:text-center italic text-grey",
            "{quote}"
        }
        p { class: "mx-4 mt-2 mb-12 md:mx-16 md:mb-16 text-right italic text-lg text-grey",
            "{author}"
        }
    }
}
