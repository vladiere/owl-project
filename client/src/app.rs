use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{console, js_sys::JsString};

use crate::{DashboardView, ForgotPassView, LoginView, MainLayout, MenuButton, NotFound, TitleBar};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    HomePage,
    #[to("/login")]
    LoginPage,
    #[to("/register")]
    RegisterPage,
    #[to("/forgot_password")]
    ForgotPassword,
    #[not_found]
    NotFound,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let show_menu = create_signal(cx, true);
    let set_auth = create_signal(cx, String::new());

    provide_context_ref(cx, set_auth);

    let update = create_ref(cx, move |_| {
        let value = if *show_menu.get() { false } else { true };

        show_menu.set(value);
    });

    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=move |cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="h-screen w-screen bg-gray-200 dark:bg-neutral-950 dark:text-neutral-300") {
                        (match route.get().as_ref() {
                            AppRoutes::HomePage => view! { cx,
                                div(class="h-full w-full flex flex-row") {
                                    MainLayout(value=show_menu)
                                    div(class="flex flex-col p-2 w-full") {
                                        div(class="flex flex-row gap-2 w-full h-10% items-center pb-2 border-b-2 border-slate-300") {
                                            MenuButton(updater=update)
                                            TitleBar(bar_title="Dashboard")
                                        }
                                        DashboardView {}
                                    }
                                }
                            },
                            AppRoutes::LoginPage => view! { cx, LoginView {} },
                            AppRoutes::RegisterPage => view! { cx, "This is the RegisterPage" },
                            AppRoutes::ForgotPassword => view! { cx, ForgotPassView {} },
                            AppRoutes::NotFound => view! { cx, NotFound {} },
                        })
                    }
                }
            }
        )
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    sycamore::render(App)
}
