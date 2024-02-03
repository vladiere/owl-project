use sycamore::prelude::*;

#[component(inline_props)]
pub fn OwlLogo<G: Html>(cx: Scope, img_class: &'static str) -> View<G> {
    view! { cx,
        img(src="public/images/owl-logo.png",class=img_class,alt="OWL Logo")
    }
}
