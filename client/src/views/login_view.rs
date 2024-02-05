use sycamore::prelude::*;
use sycamore::rt::Event;
use sycamore_router::navigate;

use crate::{alerts::DangerAlert, OwlLogo};

#[component]
pub fn LoginView<G: Html>(cx: Scope) -> View<G> {
    let username = create_signal(cx, String::new());
    let password = create_signal(cx, String::new());
    let authenticated = use_context::<Signal<i32>>(cx);

    // Errors
    let title = create_signal(cx, "");
    let message = create_signal(cx, "");

    let onsubmit = move |ev: Event| {
        ev.prevent_default();

        if username.get().is_empty() && password.get().is_empty() {
            return;
        }

        if username.get() == "kulas.admin".to_string().into() {
            if password.get() == "kulas123".to_string().into() {
                let value = format!("{}", username.get());
                authenticated.set(1);
                navigate("/");
            } else {
                message.set("Your password is incorrect");
                title.set("Password!");
            }
        } else {
            message.set("You have entered an incorrect username");
            title.set("Username!");
        }
    };

    view! { cx,
        (if title.get().is_empty() && message.get().is_empty() {
            view! { cx ,}
        }else {
            view! { cx,
                DangerAlert(title=&title.get(),message=&message.get())
            }
        })

        div(class="h-full w-full flex items-center justify-center") {
            form(class="w-[25%] flex flex-col gap-5", on:submit=onsubmit) {
                div(class="flex flex-col items-center justify-center my-3") {
                    OwlLogo(img_class="w-[60%] rounded-[50%]")
                    span(class="font-bold text-4xl") { "Login to OWL" }
                }
                div(class="flex flex-col gap-2") {
                    div(class="mb-6") {
                        label(for="username", class="block mb-2 text-sm font-medium text-gray-900 text-white") { "Username" }
                        input(type="text", id="username", class="bg-transparent border-0 border-b-2 outline-none border text-sm focus:border-blue-500 block w-full p-2.5 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 ocus:border-blue-500", placeholder="e.g john123", required=true, bind:value=username)
                    }
                    div {
                        label(for="password", class="block mb-2 text-sm font-medium text-gray-900 text-white") { "Password" }
                        input(type="password", id="password", class="bg-transparent border-0 border-b-2 outline-none border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500", placeholder="•••••••••", required=true, bind:value=password)
                    }
                }
                div(class="flex flex-col items-end gap-5") {
                    a(href="/forgot_password", class="text-6md font-medium hover:underline hover:italic") { "Forgot Password?" }
                    button(type="submit", class="text-white focus:ring-4 focus:outline-none font-medium text-lg w-full sm:w-auto px-10 py-1.5 text-center bg-blue-600 hover:bg-blue-700 focus:ring-blue-800") { "Login" }
                }
            }
        }
    }
}
