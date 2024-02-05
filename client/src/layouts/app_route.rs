use sycamore::prelude::*;
use sycamore_router::{navigate_replace, HistoryIntegration, Route, Router};
use web_sys::{console, js_sys::JsString};

use crate::{DashboardView, ForgotPassView, LoginView, MainView, NotFound};

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
pub fn AppRoute<G: Html>(cx: Scope) -> View<G> {
    let auth = use_context::<Signal<i32>>(cx);

    console::log_1(&JsString::from(format!("{}", auth.get())));

    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=move |cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="h-screen w-screen bg-neutral-950 text-neutral-300") {
                        (match route.get().as_ref() {
                            AppRoutes::HomePage => {
                                if auth.get() == 0.into() {
                                    navigate_replace("/login");
                                    view! { cx, LoginView {} }
                                } else {
                                    view! { cx,
                                        div(class="h-full w-full flex flex-row") {
                                            MainView {
                                                DashboardView {}
                                            }
                                        }
                                    }
                                }
                            },
                            AppRoutes::LoginPage => {
                                if auth.get() != 0.into() {
                                    view! { cx,
                                        div(class="h-full w-full flex flex-row") {
                                            MainView {
                                                DashboardView {}
                                            }
                                        }
                                    }
                                } else {
                                    view! { cx, LoginView {} }
                                }
                            },
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
