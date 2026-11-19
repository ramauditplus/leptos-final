use leptos::html::Input;
use leptos::prelude::*;

#[derive(Debug, Clone)]
struct LogContext(RwSignal<Vec<String>>);

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let log = RwSignal::<Vec<String>>::new(vec![]);
    let logged = move || log.get().join("\n");

    provide_context(LogContext(log));

    view! {
        <CreateAnEffect />
        <pre>{logged}</pre>
    }
}

#[component]
fn CreateAnEffect() -> impl IntoView {
    let (first, set_first) = signal(String::new());
    let (last, set_last) = signal(String::new());
    let (use_last, set_use_last) = signal(true);

    Effect::new(move |_| {
        leptos::logging::log!(
            "{}",
            if use_last.get() {
                format!("{} {}", first.get(), last.get())
            } else {
                format!("{}", first.get())
            }
        )
    });

    view! {
        <h1>
            <code>"create_effect"</code>
            " Version"
        </h1>
        <form>
            <label>
                First
                <input
                    type="text"
                    name="first"
                    prop:value=first
                    on:change:target=move |ev| set_first.set(ev.target().value())
                />
            </label>
            <label>
                Last
                <input
                    type="text"
                    name="last"
                    prop:value=last
                    on:change:target=move |ev| set_last.set(ev.target().value())
                />
            </label>
            <label>
                "Do you want to use last name ?"
                <input
                    type="checkbox"
                    name="use_last"
                    prop:checked=use_last
                    on:change:target=move |ev| set_use_last.set(ev.target().checked())
                />
            </label>
        </form>
    }
}
