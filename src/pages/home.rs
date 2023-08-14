use crate::Route;
use crate::components::{container::Container, column::Column, quote::Quote};
use dioxus::prelude::*;
use dioxus_router::prelude::*;


#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            // First paragraph
            p { class: "mb-4",
                "This is the WEBSITE of " b { "Nathan Barry" } ". I am a Computer Science and Mathematics major at " 
                Link {
                    class: "underline underline-offset-4 hover:text-highlight",
                    to:"https://www.utexas.edu/",
                    "UT Austin"
                }
                ". I enjoy machine learning, pure math, and coding things in " span { class: "text-highlight","rust."} 
            }
            p { 
                "This entire website is written in rust which allows me to show some of my projects here via WebAssembly. You can also read some of my blog posts below."
            }

            // Quote
            Quote {
                quote: String::from("\"I believe that a man should strive for only one thing in life, and that, is to have a touch of greatness\""),
                author: String::from("Félix Martí-Ibáñez")
            }


            // Links
            div { class: "flex flex-col md:flex-row md:flex-wrap gap-x-8",
                Column {
                    Container {
                        header: String::from("PERSONAL"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"https://github.com/nathan-barry" {},
                                "[Github]"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::Books {},
                                "[Favorite Books]"
                            }
                        },
                    }
                }

                Column {
                    Container {
                        header: String::from("PROJECTS"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::GameOfLife {},
                                "[Conway's Game of Life]"
                            }
                        },
                    }
                }
            }

            // Blog
            h1{ class: "mt-16", "BLOG"}

            div { class: "flex flex-col md:flex-row md:flex-wrap gap-x-8",
                Column {
                    Container {
                        header: String::from("RECENT"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "[Place Holder]"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "[Place Holder]"
                            }
                        },
                    }
                }

                Column {
                    Container {
                        header: String::from("RUST"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "[Place Holder]"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "[Place Holder]"
                            }
                        },
                    }
                }

                Column {
                    Container {
                        header: String::from("MISC"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "[Place Holder]"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "[Place Holder]"
                            }
                        },
                    }
                }
            }
        }
    }
}
