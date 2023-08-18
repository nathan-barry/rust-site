use dioxus::prelude::*;

#[inline_props]
pub fn Column<'a>(cx: Scope, header: String, children: Element<'a>) -> Element {
    render! {
        div { class: "mt-8 md:w-1/2 lg:w-1/3",
            div {
                class: "md:mr-6",
                h2 { 
                    "{header}"
                }
                div { class: "border-b" }
                div { class: "md:ml-4 mt-4 mb-8 md:mb-16",
                    children
                }
            }
        }
    }
}
