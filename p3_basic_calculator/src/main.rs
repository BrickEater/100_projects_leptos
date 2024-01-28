use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (equation, set_equation) = create_signal(String::new());
    let (solution, set_solution) = create_signal(String::new());
    view! {
        <Buttons symbol="1".to_string() setter=set_equation/>
        <Buttons symbol="2".to_string() setter=set_equation/>
        <Buttons symbol="3".to_string() setter=set_equation/>
        <Buttons symbol="4".to_string() setter=set_equation/>
        <Buttons symbol="5".to_string() setter=set_equation/>
        <Buttons symbol="6".to_string() setter=set_equation/>
        <Buttons symbol="7".to_string() setter=set_equation/>
        <Buttons symbol="8".to_string() setter=set_equation/>
        <Buttons symbol="9".to_string() setter=set_equation/>
        <Buttons symbol="*".to_string() setter=set_equation/>
        <Buttons symbol="/".to_string() setter=set_equation/>
        <Buttons symbol="+".to_string() setter=set_equation/>
        <Buttons symbol="-".to_string() setter=set_equation/>
        <Eval_Button symbol="=".to_string() getter_equation=equation setter_solution=set_solution/>
        <Clear_Button symbol="C".to_string() setter=set_equation/>

        <br/>
        <Display getter=equation getter_solution=solution/>
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
fn Eval_Button(
    symbol: String,
    getter_equation: ReadSignal<String>,
    setter_solution: WriteSignal<String>,
) -> impl IntoView {
    let symbol_clone = symbol.clone();
    view! {
        <button on:click= move |_| {
            match getter_equation.get().parse::<f32>() {
                Ok(num) => {setter_solution.update(|n| *n = "Numbers".to_string())},
                Err(_) => {setter_solution.update(|n| *n = "Numbers and stuff".to_string())}
            }
        }>{symbol_clone}</button>
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
fn Display(getter: ReadSignal<String>, getter_solution: ReadSignal<String>) -> impl IntoView {
    view! {
        <p>
            {getter}
        </p>
        </br>
        <p>
            {getter_solution}
        </p>
    }
}

fn main() {
    mount_to_body(App)
}
