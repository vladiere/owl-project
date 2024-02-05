use serde::{Deserialize, Serialize};
use sycamore::{futures::spawn_local_scoped, prelude::*};
use web_sys::{console, js_sys::JsString};

const API_TEST_URL: &str = "https://jsonplaceholder.typicode.com/users";

#[derive(Deserialize, Serialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    suits: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Company {
    name: String,
    catch_phrase: String,
    bs: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Serialize, Deserialize)]
struct Users {
    users: Vec<User>,
}

#[component]
pub fn DashboardView<G: Html>(cx: Scope) -> View<G> {
    spawn_local_scoped(cx, async move {
        let client = reqwest::Client::new();

        let result = client.get(API_TEST_URL).send().await.unwrap().text().await.unwrap();

        console::log_1(&JsString::from(format!("{result:?}")));
    });
    view! { cx,
        "Hello dashboard"
    }
}
