mod app;
mod assets;
mod components;
mod functions;
mod layouts;
mod views;

pub use app::*;
pub use assets::*;
pub use components::*;
pub use functions::*;
pub use layouts::*;
pub use views::*;
use web_sys::{window, Window};

pub fn get_local_storage_item(key: &str) -> Option<String> {
    if let Some(local_storage) = Window::local_storage().unwrap() {
        local_storage.get_item(key).unwrap()
    } else {
        None
    }
}
