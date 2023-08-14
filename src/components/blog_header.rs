use dioxus::prelude::*;

#[inline_props]
pub fn BlogHeader(cx: Scope, title: String, subtitle: String, attributes: String) -> Element {
    render! {
        div {
            h1 { class: "mt-24 mb-8 text-center", "{title}" },
            p { class: " text-center", "{subtitle}"}
            div { class: "mt-8 mb-24 italic text-grey text-center", "{attributes}" }
        }
    }
}
