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
                "THIS IS THE WEBSITE of " b { "Nathan Barry" } ". There are many like it, but this one is " i { "mine" } ". I am a Computer Science and Mathematics major at " 
                Link {
                    class: "underline underline-offset-4 hover:text-highlight",
                    to:"https://www.utexas.edu/",
                    "UT Austin"
                }
                "."
            }
            p { 
                "This entire website is written in "
                Link {
                    class: "underline underline-offset-4 hover:text-highlight",
                    to:"https://www.arewewebyet.org/",
                    "Rust"
                }
                " which allows me to show some of my projects here via WebAssembly. You can also read some of my blog posts below."
            }

            // Quote
            Quote {
                quote: "\"I believe that a man should strive for only one thing in life, and that, is to have a touch of greatness\"".into(),
                author: "Félix Martí-Ibáñez".into()
            }



            div { class: "flex flex-col md:flex-row md:flex-wrap",
                // PERSONAL
                Column {
                    Container {
                        header: "PERSONAL".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::AboutMe {},
                                "\u{2727} About Me"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::Books {},
                                "\u{2727} Favorite Books"
                            }
                        },
                    }
                }
                // PROJECTS
                Column {
                    Container {
                        header: "PROJECTS".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::GameOfLife {},
                                "\u{2727} Conway's Game of Life"
                            }
                        },
                    }
                }
                Column {
                    Container {
                        header: "LINKS".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"https://github.com/nathan-barry" {},
                                "\u{2751} Github"
                            }
                        },
                    }
                }
            }


            h2 { class:"mt-6 md:mb-12 text-center", "\u{2735} \u{2735} \u{2735}"}
            

            // BLOG
            div { class: "flex flex-col md:flex-row md:flex-wrap",
                // RECENT
                Column {
                    Container {
                        header: "RECENT".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::GEBTransformers {},
                                "\u{2727} Gödel, Escher, Bach & Transformers"
                            }
                        },
                    }
                }

                // RUST
                Column {
                    Container {
                        header: "RUST".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "\u{2727} Website Implementation"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"/" {},
                                "\u{2727} Conway's Game of Life"
                            }
                        },
                    }
                }

                // LIFE
                Column {
                    Container {
                        header: "LIFE".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::SummerPlan2023 {},
                                "\u{2727} 2023 Summer Plan"
                            }
                        },
                    }
                }

                Column {
                    Container {
                        header: "ML".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::GEBTransformers {},
                                "\u{2727} Gödel, Escher, Bach & Transformers"
                            }
                        },
                    }
                }

                // MISC
                Column {
                    Container {
                        header: "MISC".into(),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:Route::ThisMountainWeClimb {},
                                "\u{2727} This Mountain We Climb"
                            }
                        },
                    }
                }

            }
        }
    }
}
