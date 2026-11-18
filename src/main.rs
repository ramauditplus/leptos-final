use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(BindApp);
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal("ControlledInput".to_string());
    view! {
        <input
            on:input:target=move |ev| {
                set_name.set(ev.target().value());
            }
            prop:value=name
        />
        <p>{name}</p>
    }
}

#[component]
fn BindApp() -> impl IntoView {
    let (name, set_name) = signal("ControlledInput".to_string());
    let email = RwSignal::new("".to_string());
    let fav_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(false);

    view! {
        <input type="text" bind:value=(name, set_name)/>
    }
}