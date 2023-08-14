use dioxus::prelude::*;

#[inline_props]
pub fn Book(
    cx: Scope,
    number: u32, 
    title: String, 
    author: String, 
    special: bool, 
    description: String
) -> Element {
    render! {
        p { class: "m-4",
            "{number}. " b{"{title}"} " - {author}"
        }
    }
}
