use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{
    text_box::TextBox,
    blog_header::BlogHeader,
    quote::Quote,
};

#[inline_props]
pub fn AboutMe(cx: Scope) -> Element {
    render! {
        BlogHeader {
           title: "WHO IS NATHAN BARRY?".into(),
           subtitle: "A question that apperently came to you".into(),
           attributes: "Try \"How is Nathan Barry?\" next time".into()
        }

        // SYNOPSIS
        div {
            class: "flex flex-col md:flex-row",
            TextBox {
                header: "SYNOPSIS".into(),

                p { 
                    class: "mt-8",
                    style: "white-space: pre-line",
                    
                    "I am a Computer Science and Mathematics major at "
                    Link {
                        class: "underline underline-offset-4 hover:text-highlight",
                        to:"https://www.utexas.edu/",
                        "UT Austin"
                    }

                    ".

                    I was born in "
                    Link {
                        class: "underline underline-offset-4 hover:text-highlight",
                        to:"https://www.utexas.edu/",
                        "Saint Joseph, Missouri"
                    }

                    ". I started programming in 2017 my freshman year of high school, with Python and Javascript, serving PyTorch models with Flask and React.

                    In my free time I enjoy climbing, biking, and playing ultimate frisbee with the "
                    Link {
                        class: "underline underline-offset-4 hover:text-highlight",
                        to:"https://www.utexas.edu/",
                        "CS Ultimate team"
                    }

                    ".

                    I am also a pepper farmer. I am currently growing 4 strains of Habenero Pepper and one variant of Ghost Pepper"


                }
            }

            
            // IMAGE
            div {
                class: "mt-8 md:mt-0 ml-4 items-center flex flex-col min-w-[300px]",

                img { class: "border border-lightgrey", src: "me.jpg", width: "300"}
                div { 
                    class: "mt-4 text-center max-w-[300px]",
                    "Me in the GDC elevator, August 2nd 2023, 5:23 AM" }
            }
        }

        div {
            class: "mt-8 md:mt-16 flex flex-col md:flex-row",
            // INTERESTS
            div {
                class: "mt-8 md:w-1/2 lg:w-2/3",
                TextBox {
                    header: "INTERESTS".into(),
                    p { 
                        class: "mt-8",
                        style: "white-space: pre-line",
                        
                        "The great thing about life is that I happen to be solely in what I'm studing. Most of my time is taken with the workload I take in school, and that makes me happy. I'd have it no other way. 

                        At the time of writing this, I'm talking Algorithms & Complexity, Algebraic Structures I, Numerical Analysis, and Geometric Foundations of Data Science. Lovely. 

                        With whatever free time I do have, I enjoy working in rust. It is a beautiful language with beautiful tooling."

                    }
                }
            }
            // PERSONAL RECORDS
            div {
                class: "mt-8 md:ml-4 md:w-1/2 lg:w-1/3",
                TextBox {
                    header: "PERSONAL RECORDS".into(),
                    p {class: "mt-8", "\u{2727} 1600m - 4:28"}
                    p {class: "mt-4", "\u{2727} 3200m - 9:51"}
                    p {class: "mt-4", "\u{2727} 5k - 15:56"}
                    p {class: "mt-4", "\u{2727} Bench - 250 lbs"}
                    p {class: "mt-4", "\u{2727} Bouldering Grade - V8"}
                    p {class: "mt-4", "\u{2727} Balls Juggled - 4"}
                    p {class: "mt-4", ""}
                }
            }

        }

        div {
            class: "mt-12 md:mt-16 flex flex-col md:flex-row",
            TextBox {
                header: "LIFE PHILOSOPHY".into(),

                p { 
                    class: "mt-8",
                    style: "white-space: pre-line",

                    "My life philosphy started crystalizing when I was 15, and has remained more or less constant ever since. Below is my attempt to vocalize what I believe.

                    One of the main things that separates humans from other animals is that we are cognisant that time passes and the concept of a future. We acknowledge that we can sacrifice today to make a better tomorrow. This happens at all levels: your life and the generations beyond you.

                    We aren’t just minds inhabiting a body in the current moment, but a community of different versions of ourselves propagated across time. You aren't just who you are now, but also you a day from now, a year, a decade, etc. What you choose to do today will directly affect all of those future versions of you. One should act in accordance with whatever will maximize the expected value of fulfillment across all of those versions, not just the version of you today.

                    I recall once hearing the quote, \"Bees make honey. Bevers build dams. Humans create progress\". Progress seems like the main motif of humanity. I think that being able to create new things which are recognized to have value is a fundamental characteristic of humans and necessary to live a fulfilled life. The only thing that allows someone to create something new and of value is competence and hard work. The bar is always being pushed. It is at the edge of human knowledge that this value is captured.

                    I believe that hard work and competence are the only things that will lead to that outcome. Competence is the knowledge and ability to take action and get something done. The more a person can do, the more value they can provide and catch for and from society. Everyone should desire to be a competent individual in as many fields as possible and put their competence to good use.

                    It is the word \"MORE\" which drives the human race, from individuals to corporations to states, societies, and humanity as a whole. On an individual level, one must become more than who they are. Becoming better as time passes is what characterizes life, and it is this process that is innately twined with fulfillment.

                    Life should follow the format of the archetypal hero’s journey, a process of self-growth. One should adopt as much responsibility as one can and always be pushing beyond what they think they are capable are. As humanity drives forward, so must we. We must do our part to make our lives better, and the lives of all generations going forward. It is up to us to become who we could be."
                }
                
                Quote {
                    quote: "His answer to every problem, every setback, was “I will work harder!” – which he had adopted as his personal motto.".into(),
                    author: "George Orwell, Animal Farm".into()
                }
            }

        }
    }
}
