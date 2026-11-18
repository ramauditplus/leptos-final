use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App);
}


#[component]
fn App() -> impl IntoView{
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 == 1;

    // let message = move || {
    //     if is_odd(){
    //         Some("Ding ding ding")
    //     }else{
    //         None
    //     }
    // };

    // let message = move || is_odd().then(|| "Ding dong ding dong");

    let message = move || {
        match &value.get() {
            0 => "Zero",
            1 => "One",
            n if is_odd() => "Odd",
            _ => "Even"
        }
    };

    // let value_size = move || if value.get() > 5{
    //     leptos::logging::log!("{} rendering big", value.get());
    //     // "big"
    //     Big().into_any()
    // }else{
    //     leptos::logging::log!("{} rendering small", value.get());
    //     // "small"
    //     Small().into_any()
    // };

    // view! { <p>{move || { if is_odd() { "Odd" } else { "Even" } }}</p> }

    view! {
        <button on:click=move |_| {
            set_value.update(|value| *value += 1);
        }>"increment"</button>
        <button on:click=move |_| {
            set_value.update(|value| *value -= 1);
        }>"decrement"</button>
        // <p>{value_size}</p>
        <Show when=move || { value.get() > 5 } fallback=|| view! {<Small/>}>
            <Big />
        </Show>
        <p>{value}{message}</p>
    }
}

#[component]
fn Big() -> impl IntoView{
    leptos::logging::log!("Big called ");
    view! { <p>"BIG"</p> }
}

#[component]
fn Small() -> impl IntoView{
    leptos::logging::log!("Small called ");
    view! { <p>"SMALL"</p> }
}