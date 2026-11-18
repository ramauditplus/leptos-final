use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
    // value: i32,
}

#[component]
fn App() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: RwSignal::new(10),
            // value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: RwSignal::new(20),
            // value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: RwSignal::new(30),
            // value: 30,
        },
    ]);

    view! {
        <button on:click=move |_| {
            // set_data
            //     .update(|data| {
            //         for row in data {
            //             row.value *= 2;
            //         }
            //     });
            for row in &*data.read() {
                row.value.update(|value| *value *= 2);
            }
            leptos::logging::log!("{:?}", data.get());
        }>"Update values"</button>

        // <For each=move || data.get() key=|state| state.key.clone() let(child)>
        // <p>{child.value}</p>
        // </For>

        // <For
        // each=move || data.get()
        // key=|state| state.key.clone()
        // children=|child| view! { <p>{move || child.value}</p> }
        // />

        // change the key
        <For
            each=move || data.get()
            key=|state| (state.key.clone(), state.value)
            children=|child| view! { <p>{child.value}</p> }
        />
    }
}
