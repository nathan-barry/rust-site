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
    let toggle = use_state(cx, || false);
    let styles = match (*special, description) {
        (s, d) if s && d.len() != 0 => "mt-4 mb-4 text-highlight hover:font-bold",
        (s, _) if s => "mt-4 mb-4 text-highlight",
        (_, d) if d.len() != 0 => "mt-4 mb-4 hover:font-bold",
        (_, _) => "mt-4 mb-4"
    };
    let star = if description.len() != 0 {" *"} else {""};

    if *toggle.get() && description.len() != 0 {
        render! {
            div {
                onclick: move |_| toggle.set(!toggle),
                class: styles,
                "{number}. " b{"{title}"} " - {author} *"
            }
            div { class:"ml-2 md:ml-8 mb-8",
                onclick: move |_| toggle.set(!toggle),
            style: "white-space: pre-line", "{description}" }
        }
    } else {
        render!{
            div {
                onclick: move |_| toggle.set(!toggle),
                    class: styles,
                    "{number}. " b{"{title}"} " - {author}{star}"
            }
        }
    }
}
