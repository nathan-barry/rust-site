use dioxus::prelude::*;
use crate::components::text_box::TextBox;

#[inline_props]
pub fn Column<'a>(cx: Scope, header: String, children: Element<'a>) -> Element {
    render! {
        div { class: "mt-8 md:w-1/2 lg:w-1/3",
            TextBox {
                header: header.to_string(),
                children
            }
        }
    }
}
