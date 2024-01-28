//The components documentation
//https://docs.rs/leptos/latest/leptos/attr.component.html
//
//You can see props are simply arguments in the component signature
//When called in the App component, the `.to_string()` method needs to be called for strings.
//
//If a parameter is defined it is required to be used unless the `#[prop(optional)]` attribute is
//used
//
//If any argument is removed besides `optional_text` the compiler will throw an error

use leptos::*;

#[component]
fn Banner(
    number: i32,
    button_number: u8,
    text: String,
    button_text: String,
    #[prop(optional)] optional_text: String,
) -> impl IntoView {
    view! {
        <h1>"This is my banner but also the number: " {number}</h1>
        <p>"Here is some text I set " {text}</p>
        <button>{button_number}{button_text}</button>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Banner number=12 button_number=42 text="and here is some text I called".to_string() button_text=" this is a button".to_string()/>
    }
}

fn main() {
    mount_to_body(App)
}
