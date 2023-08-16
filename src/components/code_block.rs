use dioxus::prelude::*;

#[inline_props]
pub fn CodeBlock<'a>(cx: Scope, file: String, children: Element<'a>) -> Element {
    render! {
        pre {
            class: "p-4 mt-12 mb-12 bg-code border border-lightgrey rounded-xl",
            "{file}"
            children
        }
    }
}
