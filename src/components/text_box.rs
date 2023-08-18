use dioxus::prelude::*;

#[inline_props]
pub fn TextBox<'a>(cx: Scope, header: String, children: Element<'a>) -> Element {
    render! {
        div {
            class: "md:mr-6",
            h2 { 
                "{header}"
            }
            div { class: "border-b" }
            div { class: "md:ml-4 mt-8 mb-8 md:mb-16",
                children
            }
        }
    }
}
