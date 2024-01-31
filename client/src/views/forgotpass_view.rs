use sycamore::prelude::*;

use crate::{LeftArrow, OwlLogo};

#[component]
pub fn ForgotPassView<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="flex flex-col w-full h-screen") {
            div(class="flex flex-row items-center justify-between mx-10 my-4 px-5 border-b-2 border-gray-300 pb-3") {
                div(class="flex flex-row gap-3 items-center") {
                    OwlLogo(img_class="w-[15%] rounded-[50%]")
                    span(class="text-2xl font-medium") { "Password Recovery" }
                }
                div(class="flex flex-row gap-2") {
                    a(href="/login", class="flex flex-row items-center gap-2") {
                        LeftArrow(img_class="w-[25%]")
                        span(class="text-2xl") { "Back" }
                    }
                }
            }
            div(class="h-full items-center justify-center flex flex-col gap-3") {
                div(class="w-[35%] flex flex-col gap-5") {
                    div {
                        label(for="email_address", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") { "Put your registered email to recover your password" }
                        input(type="email", id="email_address", class="bg-transparent border-2 outline-none border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-gray-500 dark:focus:border-gray-500", placeholder="e.g John.doe@email.com")
                    }
                    button(type="button", class="self-end text-gray-950 bg-slate-200 hover:bg-slate-300 focus:ring-4 focus:outline-none focus:ring-gray-300 font-medium text-md w-full sm:w-auto px-5 py-2.5 text-center dark:focus:ring-gray-800") { "Recover" }
                }
            }
        }
    }
}
