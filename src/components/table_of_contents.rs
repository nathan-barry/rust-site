use dioxus::prelude::*;

#[inline_props]
pub fn TableOfContents<'a>(cx: Scope, children: Element<'a>) -> Element {
    render! {
        div {
            h2 { 
                "CONTENTS"
            }
            div { class: "mx-2 md:ml-4 mt-4 mb-8 md:mb-16",
                children
            }
        }
    }
}
