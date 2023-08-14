use dioxus::prelude::*;

#[inline_props]
pub fn Book(
    cx: Scope,
    number: u32, 
    title: String, 
    author: String, 
    special: bool, 
    description: String
) -> Element {
    let toggle = use_state(cx, || true);
    let styles = match (*special, description) {
        (s, d) if s && d.len() != 0 => "m-4 text-highlight hover:font-bold",
        (s, _) if s => "m-4 text-highlight",
        (_, d) if d.len() != 0 => "m-4 hover:font-bold",
        (_, _) => "m-4"
    };
    if *toggle.get() {
        render! {
            div {
                    class: styles,
                    onclick: move |_| toggle.set(!toggle),

                p { 
                    "{number}. " b{"{title}"} " - {author}"
                    if description.len() != 0 {" *"}
                },
            }
        }
    } else {
        render!{
            div {
                class: styles,
                onclick: move |_| toggle.set(!toggle),

                p { 
                    "{number}. " b{"{title}"} " - {author}"
                    if description.len() != 0 {" *"}
                },
                p { class:"m-4 mb-8", style: "white-space: pre-line", "{description}" }
            }
        }
    }
}
