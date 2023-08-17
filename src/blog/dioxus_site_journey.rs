use dioxus::prelude::*;

use crate::components::{
    text_box::TextBox,
    blog_header::BlogHeader,
    code_block::CodeBlock,
    quote::Quote,
};
use crate::Route;
use dioxus_router::components::Link;

#[inline_props]
pub fn DioxusSiteJourney(cx: Scope) -> Element {
    // render code
    use_effect(cx, (), move |_| {
      async move {
            let js_code = "hljs.highlightAll();";
            js_sys::eval(js_code).expect("Error highlighting code");
        }
    });

    render! {
        BlogHeader {
           title: "My Experience with Rust Frontend".into(),
           subtitle: "The Good, the Bad, and the Ugly. Below is an overview of the Dioxus framework, steps on how to host via Github Pages, and how to interact with JS libraries from Rust.".into(),
           attributes: "2023-08-015 * WIP".into()
        }

        // Why Rust for Frontend
        TextBox {
            header: "Why Rust for Frontend".into(),

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "For years I've been using React and Next.js for frontends. I've had 'nathan.rs' as my main domain for about a year and half and always had the itch to fulfill the name of the domain and remake my site in rust. This website is that attempt.

                The frontend rust ecosystem has come a long way in the last few years, and we are at a point where some frameworks are faster than React. While rewriting the site won't let me anything I couldn't do before, it does make working on it much more fun. I've never been a fan of TypeScript or JS but have always loved Rust. In addition to being more enjoyable to code in, any Rust code I write for other personal projects, I can compile and display on this site via web assembly."
            }
        }

        // Dioxus Overview
        TextBox {
            header: "Dioxus Overview".into(),

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "The frontend rust framework I decided to go with was "
                Link { to:"https://dioxuslabs.com/", class: "underline underline-offset-4 hover:text-highlight", "Dioxus"} 
                ". It was this or Leptos, but I chose Dioxus because it had support for desktop applications and mobile apps. One thing that is important to mention is that Dioxus has multiple ways to host a site.

                They have "
                code { "dioxus_web" }
                ", which is like your SPA (Single Page Appcilation) React app. SPAs work by sending over a large bundle of JavaScript and renders the website client side. This gives a very fluid experience but is bad for SEO relatively poor initial load times.

                They also have "
                code { "dioxus_liveview" } 
                ", which to my knowledge renders everthing server side and uses websockets to stream the server content to the client. This is good for real time multiparty apps like a live whiteboard or google docs.

                Finally, they have "
                code {"dioxus-fullstack"}
                " which does SSR (Server Side Rendering). This means that they render the HTML serverside and send it to the client, as opposed to sending JS to render it client side as with dioxus_web.

                I chose "
                code {"dioxus_web"} 
                " mainly since github pages only supports static web apps; no support for SSR sites. I'm also familiar with React, and as we know, old habits die hard.

                Now that we've covered that, let's go through the basics."
                    
            }

            CodeBlock { file: "./src/main.rs".into(), code { class: "language-rust", "{codeblock1()}" }}

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "Above is the bare-bones stripped down version of my site. If you're familiar with React or Next.js, the above should look very familiar.

                Snippet 1 contains two things. The first is a macro that supresses warnings about snake case. Normally for functions we use snake_case, but in the React ecosystem, we tend to use PascalCase for components. The second thing is just the imports. For external crates, we have dioxus and dioxus router. The imports below it are just two other pages I made in different files.

                Snippet 2 consists of the router. It is pretty self explanatory. The only thing of note is that the last one is the catch all route. Any URL that doesn't pattern match with the ones above it will be directed to this custom 404 page.

                Snippet 3 consists of the home page component I made. The base URL will route to and render this component. Dioxus uses RSX, but they do have an html macro that allows you to write a more familiar JSX. You can tell that I'm also using TailwindCSS which we'll talk about later.

                Snippet 4 is just the entree point for the web app. The function app() renders the router which is rendered by main. It's just boiler plate you should keep in main.rs.

                Dioxus also has it's own CLI tool that makes serving super easy. You just run " code {"dx serve"} " to host it locally. It should say to open "
                code { "https://localhost:8080/"}
                " to view the site. You can read more about how to get started with Dioxus "
                Link { to:"https://dioxuslabs.com/learn/0.4/getting_started", class: "underline underline-offset-4 hover:text-highlight", "here"} 
                "."
                
            }
        }

        // Into the Dark Forrest
        TextBox {
            header: "Into the Dark Forest".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "Once you leave tutorial zone, you'll quickly realize you're in troubled waters, one with poor documentation. It is up to you to find your way to sanctuary. Here is a list of things that took me a second to figure out.

                \u{2727} Why routing broke when I added Tailwind
                \u{2727} Hosting on Github Pages
                \u{2727} Loading JS libraries
                \u{2727} Cleaning up memory leaks

                Many of these difficulties were mainly due to some things being poorly documented. Luckily, their discord was invaluable for solving a lot of these issues, as there were many people have have came before me with the same questions in search of answers."
            }
        }

        // Into the Dark Forrest
        TextBox {
            header: "Fixing Routing".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "The main trouble for this was just obtuseness. To let the CLI know that you are using Tailwind, you must create a "
                code {"Dioxus.toml"} " file. Here is what is provided in the "
                Link { to:"https://dioxuslabs.com/learn/0.4/cookbook/tailwind", class: "underline underline-offset-4 hover:text-highlight", "docs"} "."
            }

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "I eventually learned that ithe Dioxus CLI was updated after this documentation was created. Since then, routing was completely reworked, and the CLI uses a newer default config. When I added Tailwind, I had to override the deployment configuration with an old config that didn't account for routing. Luckily, the solution was a simple fix, I just needed to add this line in the toml:"
            }

            CodeBlock { file: "./Dioxus.toml".into(), code { class: "language-toml", "{codeblock2()}" }}

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "That's better. We'll also talk about hosting on Github pages here because it similarly involved the toml."
            }
        }

        // Into the Dark Forrest
        TextBox {
            header: "Hosting on Github Pages".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "The easiest way to host this was to just move the build output directory to /docs and then, in the Pages settings page in the github repo, select it to build from /docs. Below is a screenshot of what it should look like."
            }

            img { class: "mt-12 mb-12 border border-lightgrey rounded-xl", src: "blog-mewrf-1.png", height: "300"}

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "Github Pages will host your site at "
                code {"http://<USERNAME>.github.io/"}
                ". If your repository is named "
                code {"<USERNAME>.github.io"} " the site will be hosted at the base URL, "
                code {"/"}
                  ". Otherwise it'll go live at "
                code {"/<REPO_NAME>"}
                ". I have my custom domain set up so it says "
                code {"http://nathan.rs/"}
                " for me."
            }

            CodeBlock { file: "./Dioxus.toml".into(), code { class: "language-toml", "{codeblock3()}" }}

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "My file tree looks something like this."
            }

            CodeBlock { file: "File Tree".into(), code { class: "language-toml", "{codeblock4()}" }}

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "The base level index.html is just because I have custom HTML I need to load highlight.js to do the syntax highlighting for the code block components. The Dioxus CLI produces a new HTML and JS file every time you build but there is a CLI option to have a custom HTML page which produces one in the base directory.

                As you can see, " code {"dx serve"} " and code " code {"dx build --release"} " now builds everything in " code {"/docs"} ", which Github Pages treats as the base directory. Easy peasy."
            }
        }

        // Into the Dark Forrest
        TextBox {
            header: "Interacting with JavaScript Libraries from Rust".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "The wasm-bindgen project provides developers with a few tools for interacting with JS and browser APIs.

                The web-sys crate contains bindings to browser web APIs. This includes things like the Document Object Model (DOM), HTML elements, browser-specific functions like window.fetch, and more. I use this extensively for my "
                Link { to:Route::GameOfLife {}, class: "underline underline-offset-4 hover:text-highlight", "Conway's Game of Life"}
                " implementation. You can find the source code "
                Link { to:"https://github.com/nathan-barry/nathan-barry.github.io/blob/3af983098dddfc21a81a3e1d66512b0babcfe031/src/projects/game_of_life.rs", class: "underline underline-offset-4 hover:text-highlight", "here"}

                ". js-sys is what we're looking for, which is quite easy to use. It allows us to interact with JavaScript code directly from a rust file.

                I mentioned above about loading a JS library via the index.html file. I included a link element which downloads highlight.js' default stylesheet and a script element that downloads the highlight.js library."
            }

            CodeBlock { file: "./index.html".into(), code { class: "language-html", "{codeblock5()}" }}

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "I ended up making my own custom code syntax theme in " code {"./public/hljs.css" } " and updated the link element to be " code {"<link rel=\"stylesheet\" href=\"hljs.css\""} " to use my new theme.

                Below is how to call the highlight.js library via Dioxus's " code {"use_effect"} " hook to highlight rendered code blocks."
            }


            CodeBlock { file: "./src/main.rs".into(), code { class: "language-rust", "{codeblock6()}" }}

            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "The above code is pretty self explanatory. Like React, Dioxus has a " code {"use_effect hook"} " that fires whenever the page loads. All we need to do to execute JS code is call " code { "js_sys::eval(js_code)" } ". Thus, we can interact with JavaScript libraries directly in rust.

                I'll talk about the memory leak I discovered when testing my Conway's Game of Life implementation and how I fixed it in a different post whenever I get around to it." 
            }
        }

        TextBox {
            header: "My Takeaways".into(),
            p { 
                class: "mt-8",
                style: "white-space: pre-line",
                "I had a very enjoyable time reimplementing my website in Rust using this framework.
                
                All of these Rust frontend libraries are very new and share the same characteristics as any frontier. There is usually poor documentation. Sometimes things just don't work. At times you'll have to dive deep into the source code to find answers. On the otherhand, the codebase is 'small' enough to where one person can comprehend it. Such a challenge is possible and rewarding. 

                I think that such projects are lovely to work on. It requires you to get your hands dirty and actually understand what is going on under the hood. In the past I was a classic React user that only knew the abstraction, not the layers of complexity beneath it. This project helped uncover some of that complexity, while teaching me the basics of WebAssembly. I can recommend giving this library a try.

                The End (for now)."

            
            }
        }
        
        Quote {
            quote: "In some ways, programming is like painting. You start with a blank canvas and certain basic raw materials. You use a combination of science, art, and craft to determine what to do with them.".into(),
            author: "Andrew Hunt".into()
        }
    }
}

fn codeblock1() -> String {
"// Snippet 1
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::{
    about_me::AboutMe,
    page_not_found::PageNotFound,
};


// Snippet 2
#[derive(Routable, Clone)]
enum Route {
    #[route(\"/\")]
    Home {},
    #[route(\"/about\")]
    AboutMe {},
    #[route(\"/:..route\")]
    PageNotFound { route: Vec<String> },
}


// Snippet 3
#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render! {
        h1 { \"Hello, World!\" }
        p { class: \"mb-4\",
            \"This is what RSX looks like! It's like JSX but Rust!\"
        }
    }
}


// Snippet 4
fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}".into()}


fn codeblock2() -> String {
"
# --snip--

[web.watcher]

index_on_404 = true

# --snip--".into()}


fn codeblock3() -> String {
"
[application]

# --snip--

# OLD DEFAULT
# out_dir = \"dist\"

# NEW 
out_dir = \"docs\"

# --snip--".into()}


fn codeblock4() -> String {
".
├── Cargo.lock
├── Cargo.toml
├── Dioxus.toml
├── docs
├── index.html
├── public
├── src
│   ├── blog
│   ├── components
│   ├── input.css
│   ├── main.rs
│   ├── pages
│   ├── projects
├── tailwind.config.js
└── target".into()}


fn codeblock5() -> String {

"<!DOCTYPE html>
<html>
<head>
  <!-- snip -->

  <link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.2.0/styles/default.min.css\">

  <!-- snip -->
</head>
<body>
  <!-- snip -->

  <script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.2.0/highlight.min.js\"></script>

  <!-- snip -->
</body>".into()}


fn codeblock6() -> String {
"
#[inline_props]
pub fn Home(cx: Scope) -> Element {

    // Calls highlight.js code when page loads
    use_effect(cx, (), move |_| {
      async move {
            let js_code = \"hljs.highlightAll();\";
            js_sys::eval(js_code).expect(\"Error highlighting code\");
        }
    });

    render! {
        h1 { \"Below is a code block\" }

        pre {
            code { class: \"language-rust\",
                \"let x: f64 = 7.0;\"
            }
        }
    }
}
".into()}

