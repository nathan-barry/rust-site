use dioxus::prelude::*;
use crate::components::book::Book;

#[inline_props]
pub fn Books(cx: Scope) -> Element {
    render! {
        // Top paragraph
        p { class: "mb-4",
            "I've always enjoyed seeing what books other people have read. Below are
            all the books I've read since middle school, roughly in order." 
        }
        p { class: "mb-16",
            "Those highlighted "
            span { class: "text-highlight", "blue"} " were those I particularly enjoyed or
            found impactful. An asterisk (*) indicates you can click to see some of my
            thoughts on the book :)"
        }

        h2 { "[2023 | Age 20]" }

        Book {
            number: 1,
            title: String::from("Test"),
            author: String::from("Test"),
            special: true,
            description: String::from(""),
        }

    }
}

