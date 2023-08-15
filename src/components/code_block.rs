use dioxus::prelude::*;

#[inline_props]
pub fn CodeBlock<'a>(cx: Scope, children: Element<'a>) -> Element {
    render! {
        pre {
            class: "p-4 mt-8 mb-8 bg-code border border-lightgrey border-dashed",
            children
        }
    }
}
