use sycamore::prelude::*;

#[component]
pub fn DashboardView<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p {
            "Dashboard"
        }
    }
}
