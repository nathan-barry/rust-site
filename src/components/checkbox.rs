use dioxus::prelude::*;

#[inline_props]
pub fn Checkbox(cx: Scope, text: String, checked: bool) -> Element {
    if *checked {
        render! {
            div { 
                class: "flex mt-1",

                div { 
                    class: "mt-1 w-4 h-4 mr-4 border flex items-center justify-center bg-grey",
                    "\u{2714}" 
                }
                "{text}"
            }
        }
    } else {
        render! {
            div { 
                class: "flex mt-1",

                div {class:"justify-center mt-1 w-4 h-4 mr-4 border"}
                "{text}"
            }
        }
    }
}
