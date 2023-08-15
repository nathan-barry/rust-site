use dioxus::prelude::*;

#[inline_props]
pub fn CodeBlock<'a>(cx: Scope, children: Element<'a>) -> Element {
    render! {
        div {
            class: "mt-8 mb-8 p-4 mr-6 rounded bg-darkgrey",
            children
        }
    }
}
