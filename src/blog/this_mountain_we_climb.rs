use dioxus::prelude::*;

use crate::components::{
    container::Container,
    blog_header::BlogHeader
};

#[inline_props]
pub fn ThisMountainWeClimb(cx: Scope) -> Element {
    render! {
        BlogHeader {
           title: "This Mountain We Climb".into(),
           subtitle: "This is a poem that I wrote my senior year of high school in AP Literature. It is one of the few written pieces I have of when I was younger. Enjoy.".into(),
           attributes: "2021-03-03 * Finished".into()
        }

        Container {
            header: "This Mountain We Climb".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",

                "Here we all are, this mountain we climb,
                the sure ascent, that lasts a lifetime,
                at the golden summit, a goal we all seek
                the meaning of life, at its Godly peak.

                Up we should go, a noble direction.
                Yet why do so many, rebel in rejection
                Up is worthwhile on this mountain we climb,
                at the apex, you’ll find all that’s sublime.

                Choose anything else and you will regret
                not putting in the work and how upset
                you’ll find yourself years down the line
                when fate and destiny don’t align.

                It’s steep and mighty, this mountain we climb.
                A difficult ascent, with wrong paradigms.
                Nothing worth doing is ever so easy
                so build yourself up and try to think deeply.

                Who and what and where and why
                am I on this earth so full and alive?
                One must be wise, of an earnest heart
                before one can begin our mortal march.

                Don't go through life with no direction,
                living through years with no reflection.
                Stray off the path and you’ll be lost,
                wandering forever and eventually quashed.

                Ye have been told to be prepared
                And those who aren’t, I say beware
                You have one life, treat it with care.
                Don’t die alone with a forty-yard stare.

                We each have God’s purpose that we’ll see unspun,
                if you don’t climb, you’ll life be undone.
                You'll find yourself old before you’ve begun
                A sad song you’ll life be, never to be sung.

                So hurry now, and climb this mountain,
                when ye still blessed a youthful fountain.
                Honorably embrace the peaks, the valleys,
                before you’re condemned to go down death’s alley."
            }
        }

    }
}
