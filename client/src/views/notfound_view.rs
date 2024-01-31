use sycamore::prelude::*;

#[component]
pub fn NotFound<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="h-full w-full flex flex-col gap-10 items-center justify-center") {
            h1(class="text-8xl font-bold") { "404" }
            span(class="text-2xl font-medium") { "The page you are looking for was not found" }
        }
    }
}
