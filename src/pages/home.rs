use crate::Route;
use crate::components::{text_box::TextBox, column::Column, quote::Quote};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            p { class: "mb-4",
                "THIS IS THE WEBSITE of " b { "Nathan Barry" }  ", a Computer Science and Mathematics major at " 
                Link {
                    class: "underline underline-offset-4 hover:text-highlight",
                    to:"https://www.utexas.edu/",
                    "UT Austin"
                }
                ". There are many like it, but this one is " i { "mine" } "."
            }
            p { class: "mb-4",
                "This entire website is written in "
                Link {
                    class: "underline underline-offset-4 hover:text-highlight",
                    to:"https://www.arewewebyet.org/",
                    "Rust"
                }
                " which allows me to show some of my projects here via WebAssembly. You can also read some of my blog posts below."
            }
            p { 
                "The bundle size is three times larger than when I used Next.js, but sometimes it's the DX that counts."
            }


            Quote {
                quote: "\"I believe that a man should strive for only one thing in life, and that, is to have a touch of greatness\"".into(),
                author: "Félix Martí-Ibáñez".into()
            }



            div { class: "flex flex-col md:flex-row md:flex-wrap",
                Column { header: "Personal".into(),
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::AboutMe {},
                            "\u{2727} About Me"
                        }
                    },
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::Books {},
                            "\u{2727} Favorite Books"
                        }
                    },
                }
                Column { header: "Web Assembly".into(),
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::GameOfLife {},
                            "\u{2727} Conway's Game of Life"
                        }
                    },
                }
                Column { header: "Links".into(),
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:"https://github.com/nathan-barry" {},
                            "\u{2751} Github"
                        }
                    },
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:"https://www.linkedin.com/in/nathanrs/" {},
                            "\u{2751} LinkedIn"
                        }
                    },
                }
            }



            h2 { class:"mt-6 md:mb-12 text-center", "\u{2735} \u{2735} \u{2735}"}
            


            div { class: "flex flex-col md:flex-row md:flex-wrap",
                Column { header: "Recent".into(),
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::ExperienceWithRustFrontEnd {},
                            "\u{2727} My Experience w/ Dioxus"
                        }
                    },
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::GEBTransformers {},
                            "\u{2727} Gödel, Escher, Bach & Transformers"
                        }
                    },
                }

                Column { header: "Rust".into(),
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::ExperienceWithRustFrontEnd {},
                            "\u{2727} My Experience w/ Dioxus"
                        }
                    },
                }

                Column { header: "Machine Learning".into(),
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::GEBTransformers {},
                            "\u{2727} Gödel, Escher, Bach & Transformers"
                        }
                    },
                }

                Column { header: "Life".into(),
                    p { class: "mt-2 text-bold hover:text-highlight",
                        Link { to:Route::SummerPlan2023 {},
                            "\u{2727} 2023 Summer Plan"
                        }
                    },
                }

                Column { header: "Misc.".into(),
                        p { class: "mt-2 text-bold hover:text-highlight",
                            Link { to:Route::ThisMountainWeClimb {},
                                "\u{2727} This Mountain We Climb"
                            }
                        },
                }

            }
        }
    }
}
