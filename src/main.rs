use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = MaybeSignal::derive(move || count() * 2);
    let html = "<p>This HTML will be injected.</p>";
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
            class:red=move || count() % 2 == 1
        >
            "Click me"
        </button>
        // now we use our component!
        <ProgressBar progress=count/>
        // use `Signal::derive()` to wrap a derived signal
        <ProgressBar progress=Signal::derive(double_count)/>

        <h1>{double_count}</h1>
        <div inner_html=html></div>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}
