use crate::Route;
use crate::components::text_box::Container;
use dioxus::prelude::*;
use dioxus_router::prelude::*;


#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            p { class: "mb-4",
                "This is the WEBSITE of " b { "Nathan Barry" } ". I am a Computer Science and Mathematics major at " 
                Link {
                    class: "underline underline-offset-4 hover:text-highlight",
                    to:"https://www.utexas.edu/",
                    "UT Austin"
                }
                ". I enjoy machine learning, pure math, and coding things in rust."
            }
            p { 
                "This entire website is written in rust, which allows me to show some of my projects here via WebAssembly. You can also find some of my blog posts."
            }

            div { class: "container mx-auto flex flex-col md:flex-row md:flex-wrap gap-x-8",
                div { class: "mt-8 lg:w-1/3",
                    Container {
                        header: String::from("PERSONAL"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"https://github.com/nathan-barry" {},
                                "[Github]"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"google.com" {},
                                "[Favorite Books]"
                            }
                        },
                    }
                }

                div { class: "mt-8 lg:w-1/3",
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

            div { class: "container mx-auto flex flex-col md:flex-row md:flex-wrap gap-x-8",
                div { class: "mt-8 lg:w-1/3",
                    Container {
                        header: String::from("RECENT"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"https://github.com/nathan-barry" {},
                                "[Introduction to Dioxus]"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"google.com" {},
                                "[Favorite Books]"
                            }
                        },
                    }
                }

                div { class: "mt-8 lg:w-1/3",
                    Container {
                        header: String::from("Rust"),
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"https://github.com/nathan-barry" {},
                                "[Github]"
                            }
                        },
                        p { class: "text-bold hover:text-highlight",
                            Link { to:"google.com" {},
                                "[Favorite Books]"
                            }
                        },
                    }
                }
            }
        }
    }
}
// <p>20 year old dev, CS & Math @ UT Austin</p>
//       <p>{"{Slowly amassing skills}"}</p>

//       <p className="mt-6 font-bold">Info:</p>
//       <Link href="https://github.com/nathan-barry">
//         <p className="hover:text-hover">- Github</p>
//       </Link>
