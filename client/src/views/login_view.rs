use sycamore::prelude::*;

use crate::OwlLogo;

#[component]
pub fn LoginView<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="h-full w-full flex items-center justify-center") {
            div(class="w-[25%] flex flex-col gap-5") {
                div(class="flex flex-col items-center justify-center my-3") {
                    OwlLogo(img_class="w-[60%] rounded-[50%]")
                    span(class="font-bold text-4xl") { "Login to OWL" }
                }
                div(class="flex flex-col gap-2") {
                    div(class="mb-6") {
                        label(for="username", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") { "Username" }
                        input(type="text", id="username", class="bg-transparent border-0 border-b-2 outline-none border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500", placeholder="e.g john123", required=true)
                    }
                    div {
                        label(for="password", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") { "Password" }
                        input(type="password", id="password", class="bg-transparent border-0 border-b-2 outline-none border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500", placeholder="•••••••••", required=true)
                    }
                }
                div(class="flex flex-col items-end gap-5") {
                    a(href="/forgot_password", class="text-6md font-medium hover:underline hover:italic") { "Forgot Password?" }
                    button(type="button", class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium text-lg w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") { "Login" }
                }
            }
        }
    }
}
