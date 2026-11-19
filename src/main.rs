use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(3000).await;
    value * 10
}

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    let async_data = LocalResource::new(move || load_data(count.get()));

    let stable_data = LocalResource::new(move || load_data(1));

    let async_result = move || {
        async_data
            .get()
            .map(|data| format!("Server returned value: {data:?}"))
            .unwrap_or_else(|| "Loading".into())
    };

    view! {
        <button on:click=move |_| set_count.update(|count| *count += 1)>"Click me"</button>
        <p>{count}</p>
        <p>"stable value: "{move || stable_data.get()}</p>
        <p>
            <code>"Async value "</code>
            {async_result}
        </p>
    }
}
