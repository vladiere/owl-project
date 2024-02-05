use sycamore::prelude::*;
use sycamore_router::{navigate, navigate_replace};
use web_sys::{console, js_sys::JsString};

#[component(inline_props)]
pub fn SideButtonLink<'a, G: Html>(
    cx: Scope<'a>,
    link_name: &'static str,
    img_src: &'static str,
    to_link: &'static str,
) -> View<G> {
    let logout = use_context::<Signal<i32>>(cx);

    let change_route = move |link_route| {
        if link_route != "/logout" {
            navigate_replace(link_route);
        } else {
            logout.set(0);
            navigate("/login");
            console::log_1(&JsString::from(format!("{}", logout.get())));
        }
    };

    view! { cx,
        (if to_link != "logout" {
            view! { cx,
                div(class="w-full hover:bg-slate-800 py-2") {
                    div(class="flex items-center justify-center") {
                        div(class="w-[60%]") {
                            button(class="bg-transparent w-full border-0 outline-none flex flex-row gap-2 items-center font-medium text-6md text-center", on:click=move |_| change_route(to_link)) {
                                img(src=img_src, class="w-[25%]")
                                (link_name)
                            }
                        }
                    }
                }
            }
        } else { view! { cx, } })
    }
}
