use dioxus::prelude::*;

#[inline_props]
pub fn Column<'a>(cx: Scope, children: Element<'a>) -> Element {
    render! {
        div { class: "mt-8 md:w-1/2 lg:w-1/3 md:max-w-[325px] lg:max-w-[250px]",
            children
        }
    }
}
