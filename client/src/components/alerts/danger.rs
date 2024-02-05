use sycamore::prelude::*;

#[component(inline_props)]
pub fn DangerAlert<G: Html>(cx: Scope, title: &'static str, message: &'static str) -> View<G> {
    view! { cx,
        div(class="absolute top-1 right-1 max-h-[200px] overflow-scroll w-[20%]") {
            div(class="flex items-center p-4 text-sm text-red-800 rounded-lg bg-gray-900 text-red-400", role="alert") {
                svg(class="flex-shrink-0 inline w-4 h-4 me-3", aria-hidden="true", xmlns="http://www.w3.org/2000/svg", fill="currentColor", viewBox="0 0 20 20") {
                    path(d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z" )
                }
                span(class="sr-only") { "Info" }
                div(class="flex flex-col gap-2") {
                    span(class="font-medium") { (title) } (message)
                }
            }
        }
    }
}