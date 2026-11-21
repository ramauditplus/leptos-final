use std::process::id;

use leptos::prelude::*;
use leptos::svg::path;
use leptos_router::components::*;
use leptos_router::hooks::{use_params, use_params_map};
use leptos_router::params::Params;
use leptos_router::path;

mod contact;
mod home;
mod page_not_found;
mod user_not_found;
mod user_profile;
mod users;

#[derive(Params, PartialEq)]
struct ContactParams {
    name: Option<String>,
}

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <h1>"Contact App"</h1>
            <nav>
                <a href="/">"Home"</a>
                <br />
                <a href="/contacts">"Contacts"</a>
            </nav>
            <main>
                <Routes fallback=|| "Not Found..">
                    <Route path=path!("/") view=|| view! { <h3>"Home"</h3> } />
                    <ParentRoute path=path!("/contacts") view=ContactList>
                        <ParentRoute path=path!(":id") view=ContactInfo>
                            <Route path=path!("") view=|| view! { <div>"(Contact-info)"</div> } />
                            <Route
                                path=path!("conversations")
                                view=|| view! { <div>"(Conversations)"</div> }
                            />
                        </ParentRoute>
                        <Route
                            path=path!("")
                            view=|| view! { <div>"Select a user to view contact info."</div> }
                        />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn ContactList() -> impl IntoView {
    view! {
        <div>
            <h3>"Contacts"</h3>
            <div>
                <A href="alice">"Alice"</A>
                <br />
                <A href="bob">"Bob"</A>
                <br />
                <A href="steve">"Steve"</A>
            </div>
            <Outlet />
        </div>
    }
}

#[component]
pub fn ContactInfo() -> impl IntoView {
    // let params = use_params_map();
    let params = use_params::<ContactParams>();
    // let id = move || params.read().get("id").unwrap_or_default();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.name.clone())
            .unwrap_or_default()
    };

    leptos::logging::log!("{:?}", id());

    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found",
    };

    view! {
        <h4>{name}</h4>
        <div>
            <div>
                <A href="" exact=true>
                    "Contact"
                </A>
                <br />
                <A href="conversations">"Conversations"</A>
            </div>
            <Outlet />
        </div>
    }
}

// #[component]
// fn App() -> impl IntoView {
//     view! {
//         <Router>
//             <nav></nav>
//             <main>
//                 <Routes fallback=|| "Page not found...">
//                     <Route path=path!("/") view=Home />
//                     <Route path=path!("/contact") view=Contact />
//                     <ParentRoute path=path!("/users") view=Users>
//                         <Route path=path!(":id") view=UserProfile />
//                         <Route path=path!("") view=UserNotFound />
//                     </ParentRoute>
//                     <Route path=path!("/*any") view=PageNotFound />
//                 </Routes>
//             </main>
//         </Router>
//     }
// }
