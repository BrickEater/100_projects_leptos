/*
* I need to make buttons for numbers 0-9, +, -, *, and /
* I should just make one component that takes a prop which inputs those values to an input bar
* From there I can just evaluate the field and spit out the answer
*
* So, I need a componenet for buttons, a component for the display field, and a componenet for the
* evaluated value.
*
* Here is an example of what props look like and how they are used:
* https://codesandbox.io/p/devbox/3-components-0-5-5vvl69?file=%2Fsrc%2Fmain.rs
*
*
*
*
*
* */

use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Buttons/>
    }
}

#[component]
fn Buttons() -> impl IntoView {
    view! {
        <button>
            "Some button"
        </button>
    }
}

fn main() {
    mount_to_body(App)
}
