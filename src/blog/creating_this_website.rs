use dioxus::prelude::*;

use crate::components::{
    container::Container,
    blog_header::BlogHeader,
    code_block::CodeBlock,
};

#[inline_props]
pub fn CreatingThisWebsite(cx: Scope) -> Element {
    let codeblock: String = String::from(
"use dioxus::prelude::*;

use crate::components::{
    container::Container,
    blog_header::BlogHeader,
    code_block::CodeBlock,
};

#[inline_props]
pub fn CreatingThisWebsite(cx: Scope) -> Element {
    let codeblock: String = String::from(\"
    \");



    render! {
        BlogHeader {
           title: \"How I made this Website\".into(),
           subtitle: \"An overview of the process of making my site.\".into(),
           attributes: \"2023-08-015 * WIP\".into()
        }

        Container {
            header: \"Overview\".into(),
            p { 
                class: \"mt-8\",
                style: \"white-space: pre-line\",

                \"This is example text. I am trying to
                    create a new code block compoent.

                Below should be a code block.\"
            }

            CodeBlock {
                code { style: \"white-space: pre-line\", \"{codeblock}\" }
            }
        }

    }
}"
);



    render! {
        BlogHeader {
           title: "How I made this Website".into(),
           subtitle: "An overview of the process of making my site.".into(),
           attributes: "2023-08-015 * WIP".into()
        }

        Container {
            header: "Overview".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",

                "This is example text. I am trying to create a new code block compoent.

                Below should be a code block."
            }

            CodeBlock {
                code { style: "white-space: pre-wrap", "{codeblock}" }
            }

            p { 
                class: "mt-8",
                style: "white-space: pre-line",

                "So there is a slight problem. Syntect doesn't work in WebAssembly because it has a dependency that relies on C code which doesn't work too well with WASM. It looks like I'm going to have to roll my own String2rsx syntax highlighter. I'll research of other solutions, but if it is what I need to do, it is what I need to do."
            }
        }
    }
}
