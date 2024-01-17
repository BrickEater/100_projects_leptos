use leptos::*;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (display_message, set_display_message) = create_signal("".to_string());
    view! {
        <label>"Celsius to Fahrenheit"</label>
        <br/>
        <input
            type="text"
            on:input=move |ev| {
                let value = event_target_value(&ev);
                match value.parse::<f32>() {
                    Ok(num) => {
                        let converted = (num * 9.0/5.0) +32.0;
                        set_display_message(format!("{}°C = {}°F", num, converted));
                    },
                    Err(_) => set_display_message(String::from("Not a number")),
                }
            }
        />
        <p>{display_message}</p>
    }
}
