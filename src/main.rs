use leptos::prelude::*;
use web_sys::MouseEvent;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (toggled, set_toggled) = signal(false);
    provide_context(set_toggled);
    view! {
        <p>"Toggled "{toggled}</p>
        <ButtonA setter=set_toggled />
        <ButtonB on_click=move |_| set_toggled.update(|x| *x = !*x) />
        <ButtonC on:click=move |_| set_toggled.update(|toggle| *toggle = !*toggle) />
        <ButtonD />
    }
}

// parent child relation
#[component]
fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|x| *x = !*x)>"toggle"</button> }
}

// use callback
#[component]
fn ButtonB(on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! { <button on:click=on_click>"Toggle 2"</button> }
}

// use event listener
#[component]
fn ButtonC() -> impl IntoView {
    view! { <button>"Toggle 3"</button> }
}

#[component]
fn Layout() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content />
        </main>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD />
        </div>
    }
}

#[component]
fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");
    view! { <button on:click=move |_| setter.update(|toggle| *toggle = !*toggle)>"Toggle 4"</button> }
}
