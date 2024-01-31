use sycamore::prelude::*;

#[component]
pub fn HomeView<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p {
            "Home page"
        }
    }
}
