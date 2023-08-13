use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render! {
        div {
            p { "20 year old dev, CS & Math @ UT Austin" },
            p { String::from("{Slowly amassing skills}") },
            p { class: "mt-6 font-bold",
                Link { to:Route::GameOfLife {},
                    "[Game]"
                }
            },
            p {
                Link { to:"google.com" {},
                    "[Github]"
                }
            },
            p {
                Link { to:"google.com" {},
                    "[Favorite Books]"
                }
            },
        }
    }
}
// <p>20 year old dev, CS & Math @ UT Austin</p>
//       <p>{"{Slowly amassing skills}"}</p>

//       <p className="mt-6 font-bold">Info:</p>
//       <Link href="https://github.com/nathan-barry">
//         <p className="hover:text-hover">- Github</p>
//       </Link>
