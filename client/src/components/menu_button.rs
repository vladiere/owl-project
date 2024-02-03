use sycamore::prelude::*;

#[component(inline_props)]
pub fn MenuButton<'a, G: Html>(cx: Scope<'a>, updater: &'a dyn Fn(i32)) -> View<G> {
    view! { cx,
        button(class="bg-transparent outline-none border-0 rounded-[2px]", on:click=move |_| (*updater)(0)) {
            img(class="w-full", src="public/icons/menu.svg")
        }
    }
}
