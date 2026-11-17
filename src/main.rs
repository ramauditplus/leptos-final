use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
            class:red=move || count.get() % 2 == 1
            class:green=move || count.get() % 2 == 0
            disabled=move || count.get() == 50
        >
            "Click me: "
            {count}
        </button>
        <progress max="50" value=count />
        <progress max="50" value=double_count/>
        <p>"Count: "{count}</p>
        <p>"Double count: "{double_count}</p>
    }
}
