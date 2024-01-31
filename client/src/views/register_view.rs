use sycamore::prelude::*;

#[component]
pub fn RegisterView<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p {
            "Register page"
        }
    }
}
