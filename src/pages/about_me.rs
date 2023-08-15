use dioxus::prelude::*;

use crate::components::{
    container::Container,
    blog_header::BlogHeader
};

#[inline_props]
pub fn AboutMe(cx: Scope) -> Element {
    render! {
        BlogHeader {
           title: "WHO IS NATHAN BARRY?".into(),
           subtitle: "A question many have asked".into(),
           attributes: "Good question".into()
        }

        Container {
            header: "About Me".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                
                ""
            }
        }

        img { src: "troll.png", width: "500", height: "500"}

    }
}
