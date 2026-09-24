mod api;
mod app;
mod copy;
mod image;
mod store;

use app::App;
use leptos::mount;

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}
