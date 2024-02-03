use sycamore::prelude::*;

#[component(inline_props)]
pub fn TitleBar<'a, G: Html>(cx: Scope<'a>, bar_title: &'static str) -> View<G> {
    view! { cx,
        span(class="font-medium text-4md") {
            (bar_title)
        }
    }
}
