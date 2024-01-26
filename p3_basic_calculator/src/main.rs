use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (something, set_something) = create_signal(String::new());
    view! {
        <Buttons symbol="1".to_string() setter=set_something/>
        <Buttons symbol="2".to_string() setter=set_something/>
        <Buttons symbol="3".to_string() setter=set_something/>
        <Buttons symbol="4".to_string() setter=set_something/>
        <Buttons symbol="5".to_string() setter=set_something/>
        <Buttons symbol="6".to_string() setter=set_something/>
        <Buttons symbol="7".to_string() setter=set_something/>
        <Buttons symbol="8".to_string() setter=set_something/>
        <Buttons symbol="9".to_string() setter=set_something/>
        <Buttons symbol="*".to_string() setter=set_something/>
        <Buttons symbol="/".to_string() setter=set_something/>
        <Buttons symbol="+".to_string() setter=set_something/>
        <Buttons symbol="-".to_string() setter=set_something/>
        <Eval_Button symbol="=".to_string()/>
        <Clear_Button symbol="C".to_string() setter=set_something/>

        <br/>
        <Display getter=something/>
    }
}

#[component]
fn Buttons(symbol: String, setter: WriteSignal<String>) -> impl IntoView {
    let symbol_clone = symbol.clone();
    view! {
        <button on:click= move |_| {setter.update(|n| n.push_str(&symbol))}>{symbol_clone}</button>
    }
}

#[component]
fn Eval_Button(symbol: String) -> impl IntoView {
    let symbol_clone = symbol.clone();
    view! {
        <button>{symbol_clone}</button>
    }
}

#[component]
fn Clear_Button(symbol: String, setter: WriteSignal<String>) -> impl IntoView {
    let symbol_clone = symbol.clone();
    view! {
        <button on:click= move |_| {setter.update(|n| *n = "".to_string())}>{symbol_clone}</button>
    }
}

#[component]
fn Display(getter: ReadSignal<String>) -> impl IntoView {
    view! {
        <p>
            {getter}
        </p>
    }
}

fn main() {
    mount_to_body(App)
}
