use sycamore::prelude::*;

use crate::{OwlLogo, SideButtonLink};

#[component(inline_props)]
pub fn MainLayout<'a, G: Html>(cx: Scope<'a>, value: &'a ReadSignal<bool>) -> View<G> {
    view! { cx,
        (if *value.get() {
            view! { cx,
                div(class="flex flex-col wrap w-[350px] items-center h-full border-r-2 border-slate-300") {
                    div(class="w-full items-center") {
                        div(class="flex flex-row justify-center items-center") {
                            OwlLogo(img_class="w-[30%] rounded-[50%]")
                            span(class="text-2xl font-bold") { "OWL" }
                        }
                        div(class="flex flex-col mt-2 w-full") {
                            SideButtonLink(link_name="Dashboard",to_link="/", img_src="public/icons/home.svg")
                            SideButtonLink(link_name="Accounts",to_link="/accounts", img_src="public/icons/key.svg")
                            SideButtonLink(link_name="RT Monitoring",to_link="/real-time-monitoring", img_src="public/icons/history.svg")
                            SideButtonLink(link_name="Reports",to_link="/reports", img_src="public/icons/report.svg")
                            SideButtonLink(link_name="Logout",to_link="/logout", img_src="public/icons/logout.svg")
                        }
                    }
                }
            }
        } else { view! { cx, } })
    }
}
