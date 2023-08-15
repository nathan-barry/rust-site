use dioxus::prelude::*;

#[inline_props]
pub fn Container<'a>(cx: Scope, header: String, children: Element<'a>) -> Element {
    render! {
        div {
            h2 { 
                "{header}"
            }
            div { class: "border-b mr-6" }
            div { class: "ml-2 md:ml-4 mt-4 mb-8 md:mb-16",
                children
            }
        }
    }
}
