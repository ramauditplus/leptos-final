use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
}

#[component]
fn App() -> impl IntoView {
    let numbers = vec![10, 20, 30];

    
    let users = vec![
        User{id: 1, name: "Rama".to_string()},
        User{id: 2, name: "John".to_string()},
        User{id: 3, name: "David".to_string()},
    ];

    let (users, set_users) = signal(users);
    let (name, set_name) = signal(String::new());

    view! {
        <h1>"Numbers"</h1>
        <ul>
            // {
            // numbers
            // .into_iter()
            // .map(|num| view! {<li>{num}</li>})
            // .collect_view()
            // }

            // <For
            // each=move || numbers.clone()
            // key=|number| *number
            // children=|number|{
            // view! {
            // <li>{number}</li>
            // }
            // }
            // />

            <For
                each=move || users.get()
                key=|user| user.id
                children=move |user| {
                    let id = user.id;
                    view! {
                        <li>
                            <strong>{user.name}</strong>
                            <button on:click=move |_| {
                                set_users
                                    .update(|user| {
                                        user.retain(|x| x.id != id);
                                    });
                            }>"Remove"</button>
                        </li>
                    }
                }
            />
            <input type="text" prop:value=name.get() on:input=move |ev| {
                set_name.set(event_target_value(&ev));
            } />
            <button on:click=move |_| {
                let id = users.get().len();
                set_users.update(|user| user.push(User { id: id as u32 + 1, name: name.get() }));
            }>"Add User"</button>
        </ul>
    }
}
