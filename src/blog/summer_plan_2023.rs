use dioxus::prelude::*;

use crate::components::{
    text_box::TextBox,
    blog_header::BlogHeader,
    checkbox::Checkbox,
};

#[inline_props]
pub fn SummerPlan2023(cx: Scope) -> Element {
    render! {
        BlogHeader {
           title: "2023 Summer Plan".into(),
           subtitle: "Below are some (if not most) of the things I plan on doing this summer.".into(),
           attributes: "2023-06-01—2023-08-14 * Work in Progress".into()
        }

        TextBox {
            header: "Books Read".into(),
            
            Checkbox {
                text: "Chip War: The Fight for the World's Most Critical Technology".into(),
                checked: true,
            }
            Checkbox {
                text: "The Everything Store: Jeff Bezos and the Age of Amazon".into(),
                checked: true,
            }
            Checkbox {
                text: "A Mind at Play: How Claude Shannon Invented the Information Age".into(),
                checked: true,
            }
            Checkbox {
                text: "How Google Works".into(),
                checked: true,
            }
            Checkbox {
                text: "The Creature from Jekyll Island: A Second Look at the Federal Reserve".into(),
                checked: true,
            }
            Checkbox {
                text: "The Joy of x: A guided Tour of Math, from One to Infinity".into(),
                checked: true,
            }
            Checkbox {
                text: "Algorithms to Live By: The Computer Science of Human Decisions".into(),
                checked: true,
            }
            Checkbox {
                text: "The Poincaré Conjecture: In Search of the Shape of the Universe".into(),
                checked: true,
            }
            Checkbox {
                text: "Gödel's Proof: Revised Edition".into(),
                checked: true,
            }
            Checkbox {
                text: "Cracking the Coding Interview".into(),
                checked: true,
            }
            Checkbox {
                text: "All The Math You Missed, But Need to Know for Graduate School".into(),
                checked: true,
            }
            Checkbox {
                text: "Einstein: His Life and Universe".into(),
                checked: true,
            }
            Checkbox {
                text: "American Prometheus: The Triumph and Tragedy of J. Robert Oppenheimer".into(),
                checked: true,
            }
            Checkbox {
                text: "The Man from the Future: The Visionary Life of John von Neumann".into(),
                checked: true,
            }
            Checkbox {
                text: "Rome: An Empire's Story".into(),
                checked: true,
            }
            Checkbox {
                text: "Gödel, Escher, Bach: an Eternal Golden Braid".into(),
                checked: false,
            }
            Checkbox {
                text: "Six Easy Pieces: Physics Taught By Richard Feynman".into(),
                checked: true,
            }
            Checkbox {
                text: "The Odyssey".into(),
                checked: true,
            }
            Checkbox {
                text: "Physics for Engineers and Scientists, Vol. 1".into(),
                checked: true,
            }
            Checkbox {
                text: "Physics for Engineers and Scientists, Vol. 2".into(),
                checked: false,
            }
            Checkbox {
                text: "The House of Morgan".into(),
                checked: true,
            }
            Checkbox {
                text: "The Rust Book".into(),
                checked: true,
            }
        }

        TextBox {
            header: "Miscellaneous".into(),

            Checkbox {
                text: "Councilor at CS Summer Academy".into(),
                checked: true,
            }
            Checkbox {
                text: "Climb pink at ABP".into(),
                checked: true,
            }
            Checkbox {
                text: "Climb white at ABP".into(),
                checked: true,
            }
            Checkbox {
                text: "Juggle 3 balls (and tricks)".into(),
                checked: true,
            }
            Checkbox {
                text: "Juggle 4 balls (no tricks)".into(),
                checked: true,
            }
            Checkbox {
                text: "Set up Lua Neovim config".into(),
                checked: true,
            }
            Checkbox {
                text: "Set up Local LaTeX setup".into(),
                checked: true,
            }
            Checkbox {
                text: "Grow peppers (Ghost and Habanero)".into(),
                checked: true,
            }
            Checkbox {
                text: "Symbolic_math rust crate".into(),
                checked: true,
            }
            Checkbox {
                text: "Make personal website in rust".into(),
                checked: true,
            }
            Checkbox {
                text: "Go through ThePrimeagen's algorithm course".into(),
                checked: false,
            }
            Checkbox {
                text: "Do a kickflip".into(),
                checked: false,
            }
        }

        TextBox {
            header: "Classes".into(),

            Checkbox {
                text: "Engineering Physics 1 (and lab)".into(),
                checked: true,
            }
            Checkbox {
                text: "Engineering Physics 2 (and lab)".into(),
                checked: true,
            }
            Checkbox {
                text: "Introduction to Number Theory".into(),
                checked: true,
            }
            Checkbox {
                text: "Texas Government (Core Requirement, tested out)".into(),
                checked: true,
            }
            Checkbox {
                text: "Macro Economics (Core Requirement, tested out)".into(),
                checked: true,
            }
            Checkbox {
                text: "Intro to Ancient Rome (Core Requirement)".into(),
                checked: true,
            }
            Checkbox {
                text: "Intro to Classical Mythology (Core Requirement)".into(),
                checked: true,
            }
            Checkbox {
                text: "American History (Core Requirement)".into(),
                checked: true,
            }
            Checkbox {
                text: "Rhetoric (Core Requirement)".into(),
                checked: true,
            }
        }

        TextBox {
            header: "Reflection".into(),
            p { style: "white-space: pre-line",
                "My summer was great. I did essentially a year's worth of classes, read a ton of books, riced up my setup ten-fold, and wrote a lot of code. I learned a tremendous amount about number theory, algebraic structures, physics, Rust, etc., and got good at climbing, juggling, and whatever I decided to do (omit skateboarding). 

                At the start of the summer, I actually didn't know what I was going to do. I planned on doing research but was unsure whether it (or a PhD) would be something I'd actually enjoy. I discovered that I could double major in Math and also do the integrated CS master's program if I just got a few summer classes out of the way, so that is what I did.

                Great decision. I'm really happy with how the summer turned out. As always, much smarter than before. May that trend continue."


            }
        }
    }
}
