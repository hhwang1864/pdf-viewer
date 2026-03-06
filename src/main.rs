mod api;
mod app;
mod components;
mod js_bindings;
mod models;
mod theme;

use app::App;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
