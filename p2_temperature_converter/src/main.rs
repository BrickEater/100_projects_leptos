use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h2>"Temperature Converter"</h2>
        <Celsius_to_Fahrenheit/>
        //<Fahrenheit_to_Celsius/>
    }
}

#[component]
fn Celsius_to_Fahrenheit() -> IntoView {
    let (degrees, set_degress) = create_signal("I'm not sure what goes here".to_string());

    view! {
        <input type="text"
            on:input= |ev| {
            set_degree(event_target_value(&ev));
            }
            prop:value=degrees
        />
        <p>"Degrees are: {degrees}"</p>
    }
}
