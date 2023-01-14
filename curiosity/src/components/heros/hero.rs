/*
    Appellation: hero <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Props)]
pub struct HeroScope {
    pub banner: String,
}

pub fn Hero(cx: Scope<HeroScope>) -> Element {
    let img = "https://images.unsplash.com/photo-1617420207078-f9cae65824d5?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1470&q=80";
    cx.render(rsx!(
        div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
            h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
                br { class: "hidden lg:inline-block" }
                "{cx.props.banner}"
            }
            p {
                class: "mb-8 leading-relaxed prose prose-invert",
                "Welcome to Puzzled, a personal Ethereum namespace where I try out new features proposed for the scsys ecosystem."
            }
            div { class: "flex justify-center",
                button {
                    class: "inline-flex prose prose-invert bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                    "About"
                }
                button {
                    class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:prose hover:prose-invert rounded text-lg",
                    "Build"
                }
            }
        }
        div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
            img {
                class: "object-cover object-center rounded",
                src: "{img}",
                referrerpolicy:"no-referrer",
                alt: "hero",
            }
        }
    ))
}