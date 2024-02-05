use sycamore::prelude::*;

use crate::{DashboardView, MainLayout, MenuButton, TitleBar};

#[derive(Prop)]
pub struct OtherProps<'a, G: Html> {
    pub children: Children<'a, G>,
}

#[component]
pub fn MainView<'a, G: Html>(cx: Scope<'a>, props: OtherProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    let show_menu = create_signal(cx, true);

    let update = create_ref(cx, move |_| {
        let value = if *show_menu.get() { false } else { true };

        show_menu.set(value);
    });

    view! { cx,
        div(class="h-full w-full flex flex-row") {
            MainLayout(value=show_menu)
            div(class="flex flex-col p-2 w-full") {
                div(class="flex flex-row gap-2 w-full h-10% items-center pb-2 border-b-2 border-slate-300") {
                    MenuButton(updater=update)
                    TitleBar(bar_title="Dashboard")
                }
                (children)
            }
        }
    }
}
