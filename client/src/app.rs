use sycamore::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::app_route::AppRoute;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let authenticated = create_signal(cx, 0i32);

    provide_context_ref(cx, authenticated);

    view! { cx,
        AppRoute {}
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    sycamore::render(App)
}
