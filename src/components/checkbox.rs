use dioxus::prelude::*;

#[inline_props]
pub fn Checkbox(cx: Scope, text: String, checked: bool) -> Element {
    if *checked {
        render! {
            div { 
                class: "flex items-center",

                div { 
                    class: "w-4 h-4 mr-4 border flex items-center justify-center bg-grey",
                    "\u{2714}" 
                }
                "{text}"
            }
        }
    } else {
        render! {
            div { 
                class: "flex items-center",

                div {class:"justify-center w-4 h-4 mr-4 border"}
                "{text}"
            }
        }
    }
}
