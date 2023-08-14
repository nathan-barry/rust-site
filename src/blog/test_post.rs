use dioxus::prelude::*;

#[inline_props]
pub fn TestPost(cx: Scope) -> Element {
    render! {
        p { "Test" }
    }
}
