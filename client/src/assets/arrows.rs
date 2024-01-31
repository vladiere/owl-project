use sycamore::prelude::*;

#[component(inline_props)]
pub fn LeftArrow<G: Html>(cx: Scope, img_class: &'static str) -> View<G> {
    view! { cx,
        img(src="public/left_arrow.svg",class=img_class,alt="Left Arrow")
    }
}

#[component(inline_props)]
pub fn RightArrow<G: Html>(cx: Scope, img_class: &'static str) -> View<G> {
    view! { cx,
        img(src="public/right_arrow.svg",class=img_class,alt="Right Arrow")
    }
}
