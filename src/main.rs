use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| DynamicList(DynamicListProps { initial_length: 5 }));
}

#[component]
fn App1() -> impl IntoView {
    let values = vec![0, 1, 2, 3];
    view! {
        <p>{values.clone()}</p>
        // <ul>{values.into_iter().map(|x| view! { <li>{x}</li> }).collect::<Vec<_>>()}</ul>
        <ul>{values.into_iter().map(|x| view! { <li>{x}</li> }).collect_view()}</ul>
    }
}

#[component]
fn App2() -> impl IntoView {
    let length = 5;
    let counter = (1..=length).map(|idx| RwSignal::new(idx));

    // let counter_button = counter
    // .map(|x| view! { <button on:click=move |_| *x.write() += 1>{x}</button> })
    // .collect_view();

    let counter_button = counter
        .map(|count| view! { <button on:click=move |_| count.update(|x| *x += 1)>{count}</button> })
        .collect_view();

    view! { <ul>{counter_button}</ul> }
}

#[component]
fn DynamicList(#[prop(into)] initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counter = (0..=initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counter);

    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig));
         });
        next_counter_id += 1;
    };
    view! {
        <div>
            <button on:click=add_counter>"add button"</button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, count)| {
                        let count = RwSignal::from(count);

                        view! {
                            <li>
                                <button on:click=move |_| count.update(|x| *x += 1)>{count}</button>
                                <button on:click=move |_| set_counters.write().retain(|(counter_id, _)| {
                                    counter_id != &id
                                })>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
