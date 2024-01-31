use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{ForgotPassView, LoginView, NotFound};

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
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="h-screen w-screen bg-gray-200 dark:bg-slate-950 dark:text-slate-300") {
                        (match route.get().as_ref() {
                            AppRoutes::HomePage => view! { cx, "This is the HomePage" },
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
