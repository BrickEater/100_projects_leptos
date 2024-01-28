//Doc for create_signal:
//https://docs.rs/leptos/latest/leptos/fn.create_signal.html
//
//Ultimately, I need to know how to make signals for different data types, as well as generics, when
// passing multiple different types to a function.
//
// In this example, count and set_count are both functions created by the create_signal reactive primative.
// They can be called like a function or passed as an argument to a different component
//
// When called, you simple need to declare if it's read or write and the data type to expect.
//
// The major thing I need to remember is components are just functions and different frameworks or
// libraries will still have to follow the Rust rules.
//
// Looking at this code now, it makes a lot of sense, even without a full grasp of Leptos.

use leptos::*;
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <Button setter=set_count/>
        <Display getter=count/>
    }
}

#[component]
fn Button(setter: WriteSignal<i32>) -> impl IntoView {
    view! {
        <button on:click=move |_| { setter.update(|n| *n += 1);}>
            "Count presses"
        </button>
    }
}

#[component]
fn Display(getter: ReadSignal<i32>) -> impl IntoView {
    view! {
        <p>
            {getter}
        </p>
    }
}

fn main() {
    mount_to_body(App)
}
