use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav></nav>
            // all our routes will appear inside <main>
            <main>
                <Routes>

                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>

            </main>
        </Router>
    }
}
